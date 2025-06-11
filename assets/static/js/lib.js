function updatePageTitle(title) {
  document.title = title;
}

function showError(message) {
  console.error(message);
  if (window.Alpine) {
    window.Alpine.store("toast").error(message);
  }
}

function shiningBtn(compId) {
  const button = document.getElementById(compId);
  if (button) {
    const shineAnimationDuration = 1000;
    const intervalDuration = 8000;
    function triggerShine() {
      if (!button.classList.contains("animate-shine-active")) {
        button.classList.add("animate-shine-active");
        setTimeout(() => {
          button.classList.remove("animate-shine-active");
        }, shineAnimationDuration);
      }
    }

    setInterval(triggerShine, intervalDuration);
  } else {
    console.error("Button with id 'shiningButton' not found.");
  }
}

async function uploadImageFromUrlToS3(imageUrl, presignedUrl, notifyBackendUrl) {
  try {
    const imageResponse = await fetch(imageUrl);
    if (!imageResponse.ok) throw new Error("Failed to fetch image from source");
    const blob = await imageResponse.blob();
    const s3Upload = await fetch(presignedUrl, {
      method: "PUT",
      body: blob,
      headers: {
        "Content-Type": blob.type,
      },
    });
    if (!s3Upload.ok) throw new Error("Failed to upload to S3");
    const backendNotify = await fetch(notifyBackendUrl, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ uploaded: true, source: imageUrl }),
    });
    if (!backendNotify.ok) throw new Error("Failed to notify backend");
    console.log("✅ Upload and notification successful");
  } catch (err) {
    console.error("❌ Upload failed", err);
  }
}

async function downloadImageWithLink(url, filename, pid) {
  let download_url;
  if (url.includes("https://replicapixel-dev.s3.eu-central-1.amazonaws.com")) {
    download_url = url;
  } else {
    try {
      const res = await fetch(`/api/images/download/${pid}`);
      if (!res.ok) throw new Error("Failed to fetch download URL");
      const data = await res.json();
      download_url = data.pre_url;
    } catch (err) {
      console.error("Error fetching download URL:", err);
      download_url = url;
      return;
    }
  }
  try {
    const link = document.createElement("a");
    link.href = download_url;
    link.download = filename || "download";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  } catch (error) {
    console.error("Error during download attempt:", error);
    alert("Sorry, the download could not be started.");
  }
}

function copyTextToClipboard(text) {
  const el = document.createElement("textarea");
  el.value = text;
  document.body.appendChild(el);
  el.select();
  document.execCommand("copy");
  document.body.removeChild(el);
}

function setupMobileMenu() {
  const triggerButton = document.getElementById("home-burger-menu-trigger");
  const sideMenu = document.getElementById("side-menu");
  const menuOverlay = document.getElementById("menu-overlay");
  const closeButton = document.getElementById("close-menu-button");
  const closeButtons = document.getElementsByClassName("close-menu-buttons");

  let isMenuOpen = false;

  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
    if (isMenuOpen) {
      sideMenu.style.transform = "translateX(0)";
      if (menuOverlay) {
        menuOverlay.style.opacity = "1";
        menuOverlay.style.pointerEvents = "auto";
      }
      document.body.style.overflow = "hidden";
    } else {
      sideMenu.style.transform = "translateX(100%)";
      if (menuOverlay) {
        menuOverlay.style.opacity = "0";
        menuOverlay.style.pointerEvents = "none";
      }
      document.body.style.overflow = "";
    }
  }

  if (triggerButton) {
    triggerButton.addEventListener("click", toggleMenu);
  }
  if (closeButton) {
    closeButton.addEventListener("click", toggleMenu);
  }
  if (closeButtons) {
    for (let i = 0; i < closeButtons.length; i++) {
      closeButtons[i].addEventListener("click", toggleMenu);
    }
  }
  if (menuOverlay) {
    menuOverlay.addEventListener("click", toggleMenu);
  }
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && isMenuOpen) {
      toggleMenu();
    }
  });
}

// async function oAuth2(provider_link) {
//   try {
//     const response = await fetch(provider_link);
//     if (!response.ok) {
//       throw new Error("Failed to get OAuth URL");
//     }
//     const oauthLink = await response.text();
//     console.log(oauthLink);
//     window.location.href = oauthLink;
//   } catch (err) {
//     console.error("OAuth2 error:", err);
//     if (window.Alpine) {
//       window.Alpine.store("toast").error("Could not start OAuth2 login. Please try again.");
//     }
//   }
// }

async function oAuth2(provider_link) {
  try {
    // Validate input
    if (!provider_link) {
      showError("Missing authentication provider URL");
      return;
    }

    // Verify the URL is properly formatted
    let authUrl;
    try {
      authUrl = new URL(provider_link);
    } catch (e) {
      showError("Invalid authentication URL");
      return;
    }

    // Add timeout to prevent hanging
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 10000); // 10 seconds

    const response = await fetch(authUrl.toString(), {
      signal: controller.signal,
      redirect: "manual", // Important for OAuth flows
    });

    clearTimeout(timeout);

    if (!response.ok) {
      showError(`Authentication service unavailable (${response.status})`);
      return;
    }

    const oauthLink = await response.text();

    // Validate the OAuth URL before redirecting
    try {
      new URL(oauthLink); // Basic validation without protocol check
    } catch (e) {
      showError("Invalid authentication response");
      return;
    }

    // Perform the redirect
    window.location.href = oauthLink;
  } catch (err) {
    console.error("OAuth2 error:", err);
    if (err.name === "AbortError") {
      showError("Authentication request timed out");
    } else {
      showError("Could not start login. Please try again.");
    }
  }
}

// async function fetchAndOpenReceipt(link, orderPid) {
//   try {
//     console.log(link, orderPid);
//     const res = await fetch(`${link}/${orderPid}`);
//     const url = await res.text();
//     console.log(url);
//     window.open(url, "_blank");
//   } catch (err) {
//     console.error("Failed to fetch receipt link:", err);
//     window.Alpine.store("toast").error("Failed to fetch stripe receipt link");
//   }
// }

async function fetchAndOpenReceipt(link, orderPid) {
  try {
    // Validate inputs
    if (!link || !orderPid) {
      if (window.Alpine) {
        window.Alpine.store("toast").error("Missing required parameters");
      }
      return;
    }

    // Sanitize orderPid to prevent injection
    const sanitizedPid = orderPid.replace(/[^a-zA-Z0-9-_]/g, "");

    // Construct safe URL
    const url = new URL(`${link}/${sanitizedPid}`);

    // // Verify the URL is HTTPS and from allowed domain(s)
    // if (!url.protocol.startsWith("https")) {
    //   if (window.Alpine) {
    //     window.Alpine.store("toast").error("Insecure connection protocol");
    //   }
    //   return;
    // }

    // Add timeout to the fetch request
    const controller = new AbortController();
    const timeoutId = setTimeout(() => {
      controller.abort();
      if (window.Alpine) {
        window.Alpine.store("toast").error("Request timed out. Please try again.");
      }
    }, 5000); // 5 second timeout

    const res = await fetch(url.toString(), {
      signal: controller.signal,
      headers: {
        "Content-Type": "text/plain",
      },
      credentials: "same-origin",
    });

    clearTimeout(timeoutId);

    if (!res.ok) {
      if (window.Alpine) {
        window.Alpine.store("toast").error(`Request failed with status ${res.status}`);
      }
      return;
    }

    const receiptUrl = await res.text();

    // Validate the response URL before opening
    if (typeof receiptUrl !== "string" || !receiptUrl) {
      if (window.Alpine) {
        window.Alpine.store("toast").error("Invalid receipt received");
      }
      return;
    }

    const receiptUrlObj = new URL(receiptUrl);

    // Only allow opening Stripe receipt URLs as an example
    if (!receiptUrlObj.hostname.endsWith("stripe.com")) {
      if (window.Alpine) {
        window.Alpine.store("toast").error("Receipt URL is not from an allowed domain");
      }
      return;
    }

    // Safely open the window with noopener for security
    const newWindow = window.open(receiptUrl, "_blank", "noopener,noreferrer");

    if (!newWindow) {
      if (window.Alpine) {
        window.Alpine.store("toast").error("Popup was blocked. Please allow popups for this site.");
      }
      return;
    }
  } catch (err) {
    console.error("Failed to fetch receipt:", err);
    if (window.Alpine) {
      window.Alpine.store("toast").error("Failed to fetch receipt. Please contact support.");
    }
  }
}
