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

// async function downloadImage(url, filename) {
//   try {
//     const response = await fetch(url, { mode: "cors" }); // Requires CORS headers on S3
//     if (!response.ok) throw new Error("Network response was not ok");

//     const blob = await response.blob();
//     const blobUrl = URL.createObjectURL(blob);

//     const link = document.createElement("a");
//     link.href = blobUrl;
//     link.download = filename;
//     document.body.appendChild(link);
//     link.click();
//     link.remove();

//     URL.revokeObjectURL(blobUrl); // Clean up
//   } catch (error) {
//     console.error("Image download failed:", error);
//   }
// }

//Todo Fix S3 Key to also include webp.
// async function convertBlobToWebP(blob, quality = 0.8) {
//   const img = new Image();
//   const imgLoad = new Promise((resolve, reject) => {
//     img.onload = () => resolve();
//     img.onerror = reject;
//   });

//   const url = URL.createObjectURL(blob);
//   img.src = url;
//   await imgLoad;
//   URL.revokeObjectURL(url);

//   const canvas = document.createElement("canvas");
//   canvas.width = img.width;
//   canvas.height = img.height;
//   const ctx = canvas.getContext("2d");
//   if (!ctx) throw new Error("Canvas 2D context not available");
//   ctx.drawImage(img, 0, 0);

//   const webpBlob =
//     ((await new Promise()) < Blob) |
//     (null > ((resolve) => canvas.toBlob(resolve, "image/webp", quality)));

//   if (!webpBlob) throw new Error("WebP conversion failed");
//   return webpBlob;
// }

// async function uploadImageFromUrlToS3(imageUrl, presignedUrl, notifyBackendUrl) {
//   try {
//     // Fetch original image blob
//     const imageResponse = await fetch(imageUrl);
//     if (!imageResponse.ok) throw new Error("Failed to fetch image from source");
//     const originalBlob = await imageResponse.blob();

//     // Convert to WebP blob
//     const webpBlob = await convertBlobToWebP(originalBlob, 0.75);

//     // Upload converted WebP blob to S3
//     const s3Upload = await fetch(presignedUrl, {
//       method: "PUT",
//       body: webpBlob,
//       headers: {
//         "Content-Type": webpBlob.type,
//       },
//     });
//     if (!s3Upload.ok) throw new Error("Failed to upload to S3");

//     // Notify backend after successful upload
//     const backendNotify = await fetch(notifyBackendUrl, {
//       method: "PATCH",
//       headers: {
//         "Content-Type": "application/json",
//       },
//       body: JSON.stringify({ uploaded: true, source: imageUrl }),
//     });
//     if (!backendNotify.ok) throw new Error("Failed to notify backend");

//     console.log("✅ Upload and notification successful");
//   } catch (err) {
//     console.error("❌ Upload failed", err);
//   }
// }
