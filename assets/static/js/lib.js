function updatePageTitle(title) {
  document.title = title;
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

async function oAuth2(provider_link) {
  try {
    const response = await fetch(provider_link);
    if (!response.ok) {
      throw new Error("Failed to get OAuth URL");
    }
    const oauthLink = await response.text();
    console.log(oauthLink);
    window.location.href = oauthLink;
  } catch (err) {
    console.error("OAuth2 error:", err);
    if (window.Alpine) {
      window.Alpine.store("toast").error("Could not start OAuth2 login. Please try again.");
    }
  }
}
