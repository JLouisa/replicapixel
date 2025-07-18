import { zipSync } from "fflate";

export function parse_num(number: number): string {
  return number < 10 ? `0${number}s` : `${number}s`;
}

export function getBaseUrl(): string {
  // const url =
  //   window.location.hostname === "localhost" ? "http://localhost:5150" : "https://replicapixel.com";
  const url = `${window.location.protocol}//${window.location.host}`;
  return url;
}

export async function createZip(files: File[], zipName: string): Promise<File> {
  const zipData: Record<string, Uint8Array> = {};

  for (const file of files) {
    zipData[file.name] = new Uint8Array(await file.arrayBuffer());
  }

  const zippedItem = new Blob([zipSync(zipData)], { type: "application/zip" });

  return new File([zippedItem], `${zipName}.zip`, {
    type: "application/zip",
    lastModified: Date.now(),
  });
}

export function createBatches(gen_img: number, limit_per_request = 4): [number, number] {
  let batch: number = Math.floor(gen_img / limit_per_request);
  let single: number = gen_img % limit_per_request;
  return [batch, single];
}

export function ensureBoolean(value: unknown): boolean {
  if (typeof value === "boolean") return value;
  if (typeof value === "string") return value.toLowerCase() === "true";
  return false;
}

export function replaceDivIfFound() {
  const targetDiv = document.getElementById("no-images") as HTMLDivElement | null;

  if (!targetDiv) {
    return;
  }

  const newDiv = document.createElement("div");
  newDiv.id = "drive-gallery";
  newDiv.className = "grid grid-cols-2 md:grid-cols-6 gap-4 w-full self-start";

  targetDiv.replaceWith(newDiv);
}

export async function extractThumbnailFileFromVideo(
  videoBlob: Blob,
  uuid: string,
  seekTime = 1
): Promise<File> {
  // 1. Fetch video as Blob
  const objectUrl = URL.createObjectURL(videoBlob);

  // 2. Create a video element
  const video = document.createElement("video");
  video.id = `video-${uuid}`;
  video.src = objectUrl;
  video.crossOrigin = "anonymous";
  video.muted = true;
  video.preload = "auto";

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

// async function uploadMediaToS3(media: Blob, presignedUrl: string) {
//   try {
//     const s3Upload = await fetch(presignedUrl, {
//       method: "PUT",
//       body: media,
//       headers: {
//         "Content-Type": media.type,
//       },
//     });
//     if (!s3Upload.ok) throw new Error("Failed to upload to S3");
//     console.log("✅ Upload and notification successful");
//   } catch (err) {
//     console.error("❌ Upload failed", err);
//   }
// }

// export async function videoProcessing(
//   videoUrl: string,
//   video_pre_url: string,
//   thumbnail_pre_url: string,
//   notifyBackendUrl: string,
//   uuid: string,
//   target: string
// ) {
//   console.log("Processing video...");
//   try {
//     const response_video = await fetch(videoUrl);
//     if (!response_video.ok) throw new Error("Video fetch failed");

//     const video = await response_video.blob();
//     const thumbnail = await extractThumbnailFileFromVideo(video, uuid);

//     await uploadMediaToS3(video, video_pre_url);
//     await uploadMediaToS3(thumbnail, thumbnail_pre_url);
//   } catch (err) {
//     console.error("❌ Media download or upload failed", err);
//     return;
//   }

//   try {
//     let swap = "innerHTML";
//     await window.htmx.ajax("GET", notifyBackendUrl, { target, swap });
//   } catch (err) {
//     console.error("❌ Backend notify or DOM update failed", err);
//   }
// }

function testConsole(foo: string) {
  console.log(foo);
}

export function videoDialog(
  videoDeleteLink: string,
  videoPid: string,
  videoUrl: string,
  title: string,
  prompt: string,
  duration: string
) {
  // console.log("videoDeleteLink:", videoDeleteLink);
  // console.log("videoPid:", videoPid);
  // console.log("videoUrl:", videoUrl);
  // console.log("title:", title);
  // console.log("prompt:", prompt);
  // console.log("duration:", duration);

  const dialogHtml = document.getElementById("video-modal") as HTMLDialogElement | null;
  const videoPlayerHtml = document.getElementById("video-player") as HTMLIFrameElement | null;
  const titleHtml = document.getElementById("video-title") as HTMLHeadingElement | null;
  const promptHtml = document.getElementById("video-prompt") as HTMLHeadingElement | null;
  const durationHtml = document.getElementById("video-duration") as HTMLHeadingElement | null;
  const downloadHtml = document.getElementById("video-download") as HTMLAnchorElement | null;
  const deleteButton = document.getElementById("video-delete-button") as HTMLButtonElement;

  if (
    !dialogHtml ||
    !videoPlayerHtml ||
    !titleHtml ||
    !promptHtml ||
    !durationHtml ||
    !downloadHtml ||
    !deleteButton
  ) {
    console.warn("Dialog or others not found.");
    return;
  }

  titleHtml.textContent = title;
  promptHtml.textContent = prompt;
  durationHtml.textContent = duration;
  videoPlayerHtml.src = videoUrl;
  downloadHtml.href = videoUrl;
  downloadHtml.download = title;

  deleteButton.onclick = () => {
    handleVideoDeletion(videoDeleteLink, videoPid);
  };

  dialogHtml.showModal();
  dialogHtml.addEventListener(
    "close",
    () => {
      videoPlayerHtml.src = "";
    },
    { once: true }
  );
}

export async function handleVideoDeletion(deleteLink: string, videoPid: string) {
  if (!confirm("Are you sure you want to delete this video?")) {
    return;
  }

  const dialogHtml = document.getElementById("video-modal") as HTMLDialogElement;
  const targetId = `#video-card-completed-${videoPid}`;
  const fullDeleteUrl = `${deleteLink}/${videoPid}`;

  await window.htmx.ajax("DELETE", fullDeleteUrl, {
    target: targetId,
    swap: "delete",
  });

  if (dialogHtml) {
    dialogHtml.close();
  }
}

// export async function deleteVideo() {
//   alert("Are you sure you want to delete this video?");
//   const dialogHtml = document.getElementById("video-modal") as HTMLDialogElement | null;
//   const deleteAnchorHtml = document.getElementById("video-delete") as HTMLAnchorElement | null;
//   if (!deleteAnchorHtml) {
//     console.warn("Delete button not found.");
//     return;
//   }
//   const deleteLink = deleteAnchorHtml.href;
//   const targetId = deleteAnchorHtml.getAttribute("hx-target");

//   console.log("Delete video link:", deleteLink);
//   console.log("Delete video id:", targetId);

//   await window.htmx.ajax("DELETE", deleteLink, {
//     target: targetId,
//     swap: "outerHTML",
//   });

//   if (dialogHtml) {
//     dialogHtml.close();
//   }
// }

export function packDialog(
  packPid: string,
  userCredits: number,
  mainImageUrl: string,
  subImageRawString: string,
  title: string,
  used: string,
  popularValue: string,
  description: string,
  creditsRawString: number,
  numImagesRawString: number,
  featuresRawString: string
) {
  // console.log("packPid", packPid);
  // console.log("mainImageUrl", mainImageUrl);
  // console.log("subImage", subImageRawString);
  // console.log("title", title);
  // console.log("isPopular", popularValue);
  // console.log("used", used);
  // console.log("description", description);
  // console.log("credits", credits);
  // console.log("numImages", numImages);
  // console.log("features", featuresRawString);

  const credits = Number(creditsRawString);
  const numImages = Number(numImagesRawString);

  const isPopular = popularValue === "true";
  const subImageArray = subImageRawString
    .replace(/[\[\]]/g, "")
    .split(",")
    .map((str) => str.trim());
  const featuresArray = featuresRawString
    .replace(/[\[\]]/g, "")
    .split(",")
    .map((str) => str.trim());

  const dialogHtml = document.getElementById("pack-modal") as HTMLDialogElement | null;
  const mainImage = document.getElementById("pack-main-image-mobile") as HTMLImageElement | null;
  const mainImageMobile = document.getElementById("pack-main-image") as HTMLImageElement | null;
  const subImages = document.getElementsByClassName("pack-images");
  const subImagesMobile = document.getElementsByClassName("pack-images-mobile");
  const titleHtml = document.getElementById("pack-title") as HTMLHeadingElement | null;
  const isPopularHtml = document.getElementById("pack-popular") as HTMLDivElement | null;
  const usedHtml = document.getElementById("pack-used") as HTMLSpanElement | null;
  const descriptionHtml = document.getElementById("pack-description") as HTMLDivElement | null;
  const creditsHtml = document.getElementById("pack-credits") as HTMLDivElement | null;
  const numImagesHtml = document.getElementById("pack-photos") as HTMLSpanElement | null;
  const features = document.getElementsByClassName("pack-features");
  const featuresHtml1 = document.getElementById("pack-feature-1") as HTMLDivElement | null;
  const featuresHtml2 = document.getElementById("pack-feature-2") as HTMLDivElement | null;
  const featuresHtml3 = document.getElementById("pack-feature-3") as HTMLDivElement | null;
  const featuresHtml4 = document.getElementById("pack-feature-4") as HTMLDivElement | null;
  const inputPackPidHtml = document.getElementById("pack-input-id") as HTMLInputElement | null;
  const formSelectionHtml = document.getElementById("pack-form-modal") as HTMLInputElement | null;

  if (!dialogHtml) return console.warn("Dialog not found.");
  if (!mainImage) return console.warn("mainImage not found.");
  if (!mainImageMobile) return console.warn("mainImageMobile not found.");
  if (subImages.length === 0) console.warn("No elements with class 'pack-images' found.");
  if (!titleHtml) return console.warn("titleHtml not found.");
  if (!usedHtml) return console.warn("usedHtml not found.");
  if (!isPopularHtml) return console.warn("isPopularHtml not found.");
  if (!descriptionHtml) return console.warn("descriptionHtml not found.");
  if (!creditsHtml) return console.warn("credits not found.");
  if (!numImagesHtml) return console.warn("numImages not found.");
  if (!featuresHtml1) return console.warn("features not found.");
  if (!featuresHtml2) return console.warn("features not found.");
  if (!featuresHtml3) return console.warn("features not found.");
  if (!featuresHtml4) return console.warn("features not found.");

  if (inputPackPidHtml) {
    inputPackPidHtml.value = packPid;
  }

  if (formSelectionHtml) {
    const isShow = userCredits > credits;
    formSelectionHtml.classList.toggle("hidden", !isShow);
  }

  mainImage.src = mainImageUrl;
  mainImageMobile.src = mainImageUrl;
  titleHtml.textContent = title;
  usedHtml.textContent = used;
  descriptionHtml.textContent = description;
  isPopularHtml.classList.toggle("hidden", !isPopular);
  creditsHtml.textContent = credits.toString();
  numImagesHtml.textContent = numImages.toString();

  const featuresToSet = Math.min(features.length, featuresArray.length);
  for (let i = 0; i < featuresToSet; i++) {
    const imgElement = features[i] as HTMLImageElement;
    imgElement.textContent = featuresArray[i];
  }
  const imagesToSet = Math.min(subImages.length, subImageArray.length);
  for (let i = 0; i < imagesToSet; i++) {
    const imgElement = subImages[i] as HTMLImageElement;
    imgElement.src = subImageArray[i];
  }
  for (let i = 0; i < imagesToSet; i++) {
    const imgElement = subImagesMobile[i] as HTMLImageElement;
    imgElement.src = subImageArray[i];
  }

  dialogHtml.showModal();
}

(window as any).videoDialog = videoDialog;
(window as any).packDialog = packDialog;
(window as any).extractThumbnailFileFromVideo = extractThumbnailFileFromVideo;
(window as any).testConsole = testConsole;
