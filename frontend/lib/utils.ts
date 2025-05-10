import { zipSync } from "fflate";

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
