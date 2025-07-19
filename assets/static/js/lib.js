function updatePageTitle(title) {
  document.title = title;
}

function replaceUrl(newUrl) {
  history.replaceState(null, "", newUrl);
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

// function downloadVideo(url) {
//   const a = document.createElement("a");
//   a.href = url;
//   a.download = "";
//   document.body.appendChild(a);
//   a.click();
//   document.body.removeChild(a);
// }

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
      showError(`Authentication service unavailable (${response.status})`);
      return;
    }
    const oauthLink = await response.text();
    try {
      new URL(oauthLink);
    } catch (e) {
      showError("Invalid authentication response.");
      return;
    }
    window.location.href = oauthLink;
  } catch (err) {
    console.error("OAuth2 error.", err);
    if (window.Alpine) {
      window.Alpine.store("toast").error("Could not start OAuth2 login. Please try again.");
    }
  }
}

async function fetchAndOpenReceipt(link, orderPid) {
  try {
    const res = await fetch(`${link}/${orderPid}`);
    if (!res.ok) {
      console.error("Failed to fetch receipt link.", res);
      window.Alpine.store("toast").error("Failed to fetch stripe receipt link.");
      return;
    }
    const url = await res.text();
    window.open(url, "_blank");
  } catch (err) {
    console.error("Failed to fetch receipt link.", err);
    if (window.Alpine) {
      window.Alpine.store("toast").error("Failed to fetch stripe receipt link.");
    }
  }
}

function handleThemeToggle(checkbox) {
  // Determine the new theme based on whether the checkbox is checked
  const newTheme = checkbox.checked ? "light" : "dark";

  // Apply the new theme to the <html> element
  document.documentElement.setAttribute("data-theme", newTheme);

  // Save the user's choice to localStorage
  localStorage.setItem("theme", newTheme);
}

function testConsole(foo) {
  console.log(foo);
}

async function extractThumbnailFileFromVideo(videoBlob, uuid, seekTime = 1) {
  // 1. Fetch video as Blob
  const objectUrl = URL.createObjectURL(videoBlob);

  // 2. Create a video element
  const video = document.createElement("video");
  video.id = `canvas-${uuid}`;
  video.src = objectUrl;
  video.crossOrigin = "anonymous";
  video.muted = true;
  video.preload = "auto";

  // Error handling cleanup
  const cleanup = () => URL.revokeObjectURL(objectUrl);

  video.onerror = () => {
    cleanup();
    // reject(new Error("Video loading failed"));
  };

  // 3. Return a File (JPEG thumbnail)
  return new Promise((resolve, reject) => {
    video.onloadedmetadata = () => {
      video.currentTime = seekTime;
    };

    video.onseeked = () => {
      const canvas = document.createElement("canvas");
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;

      const ctx = canvas.getContext("2d");
      if (!ctx) return reject("Failed to get canvas context");

      ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

      canvas.toBlob(
        (blob) => {
          if (!blob) return reject("Canvas toBlob failed");

          const file = new File([blob], `thumbnail-${uuid}.jpeg`, { type: "image/jpeg" });
          resolve(file);
        },
        "image/jpeg",
        0.9
      );
    };

    video.onerror = (e) => {
      reject("Error loading video");
    };
  });
}

async function uploadMediaToS3(media, presignedUrl) {
  try {
    const s3Upload = await fetch(presignedUrl, {
      method: "PUT",
      body: media,
      headers: {
        "Content-Type": media.type,
      },
    });
    if (!s3Upload?.ok) throw new Error("Failed to upload to S3");
  } catch (err) {
    console.error("❌ Upload failed", err);
  }
}

function updateVideoCard(htmlString, video_id) {
  // 1. Parse the new card HTML
  const template = document.createElement("template");
  template.innerHTML = htmlString.trim();
  const newCard = template.content.firstElementChild;

  // 2. Find the existing card
  const oldCard = document.getElementById(video_id);
  if (!oldCard) {
    console.error(`Card with ID ${video_id} not found`);
    return null;
  }

  // 3. Preserve important attributes from old card
  // (e.g., HTMX attributes if they exist)
  const hxAttributes = ["hx-get", "hx-trigger", "hx-swap"];
  hxAttributes.forEach((attr) => {
    if (oldCard.hasAttribute(attr)) {
      newCard.setAttribute(attr, oldCard.getAttribute(attr));
    }
  });

  // 4. Replace the card
  oldCard.replaceWith(newCard);

  // 5. Reinitialize any scripts in the new content
  const scripts = newCard.querySelectorAll("script");
  scripts.forEach((script) => {
    const newScript = document.createElement("script");
    newScript.textContent = script.textContent;
    script.replaceWith(newScript);
  });

  return newCard;
}

async function videoProcessing(
  videoUrl,
  video_pre_url,
  thumbnail_pre_url,
  notifyBackendUrl,
  video_pid
) {
  try {
    // 1. Fetch video
    const videoResponse = await fetch(videoUrl);
    if (!videoResponse.ok) throw new Error(`Video fetch failed: ${videoResponse.status}`);
    const videoBlob = await videoResponse.blob();

    // // 2. Generate thumbnail
    // const thumbnail = await extractThumbnailFileFromVideo(videoBlob, video_pid);

    // // 3. Parallel uploads
    // await Promise.all([
    //   uploadMediaToS3(videoBlob, video_pre_url),
    //   uploadMediaToS3(thumbnail, thumbnail_pre_url),
    // ]);

    // 4. Notify backend
    const completionResponse = await fetch(notifyBackendUrl);
    if (!completionResponse.ok) throw new Error("Completion notification failed");

    // // 5. Update UI
    // const html = await completionResponse.text();
    // const updatedCard = updateVideoCard(html, video_pid);

    // if (!updatedCard) {
    //   console.warn("Card update failed - falling back to HTMX");
    //   // Implement HTMX fallback here if needed
    // }
  } catch (error) {
    console.error("❌ Processing failed:", error);
    // Implement retry logic or error UI update here
  }
}
