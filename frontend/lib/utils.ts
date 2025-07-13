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

export function openVideoDialog(videoUrl: string, title: string, prompt: string, duration: string) {
  const dialogHtml = document.getElementById("video-modal") as HTMLDialogElement | null;
  const videoPlayerHtml = document.getElementById("video-player") as HTMLIFrameElement | null;
  const titleHtml = document.getElementById("video-title") as HTMLHeadingElement | null;
  const promptHtml = document.getElementById("video-prompt") as HTMLHeadingElement | null;
  const durationHtml = document.getElementById("video-duration") as HTMLHeadingElement | null;

  if (!dialogHtml || !videoPlayerHtml || !titleHtml || !promptHtml || !durationHtml) {
    console.warn("Dialog or iframe not found.");
    return;
  }

  titleHtml.textContent = title;
  promptHtml.textContent = prompt;
  durationHtml.textContent = duration;
  videoPlayerHtml.src = videoUrl;

  dialogHtml.showModal();
  dialogHtml.addEventListener(
    "close",
    () => {
      videoPlayerHtml.src = "";
    },
    { once: true }
  );
}

async function uploadMediaToS3(media: Blob, presignedUrl: string) {
  try {
    const s3Upload = await fetch(presignedUrl, {
      method: "PUT",
      body: media,
      headers: {
        "Content-Type": media.type,
      },
    });
    if (!s3Upload.ok) throw new Error("Failed to upload to S3");
    console.log("✅ Upload and notification successful");
  } catch (err) {
    console.error("❌ Upload failed", err);
  }
}

export async function videoProcessing(
  videoUrl: string,
  video_pre_url: string,
  thumbnail_pre_url: string,
  notifyBackendUrl: string,
  uuid: string,
  target: string
) {
  console.log("Processing video...");
  try {
    const response_video = await fetch(videoUrl);
    if (!response_video.ok) throw new Error("Video fetch failed");

    const video = await response_video.blob();
    const thumbnail = await extractThumbnailFileFromVideo(video, uuid);

    await uploadMediaToS3(video, video_pre_url);
    await uploadMediaToS3(thumbnail, thumbnail_pre_url);
  } catch (err) {
    console.error("❌ Media download or upload failed", err);
    return;
  }

  try {
    let swap = "innerHTML";
    await window.htmx.ajax("GET", notifyBackendUrl, { target, swap });
  } catch (err) {
    console.error("❌ Backend notify or DOM update failed", err);
  }
}

function testConsole(foo: string) {
  console.log(foo);
}

(window as any).openVideoDialog = openVideoDialog;
(window as any).videoProcessing = videoProcessing;
(window as any).extractThumbnailFileFromVideo = extractThumbnailFileFromVideo;
(window as any).testConsole = testConsole;
