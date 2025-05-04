import Dropzone from "dropzone";
import "dropzone/dist/dropzone.css";

// Internal state for validated files for the instance created by DropzoneSetup
let validatedFiles: File[] = [];

// Internal async validation function
function handleFileAsync(file: File): Promise<void> {
  return new Promise((resolve, reject) => {
    const isValid = ["image/jpeg", "image/png"].includes(file.type);
    console.log(`Standalone validating ${file.name}. Is valid: ${isValid}`);
    setTimeout(() => {
      if (isValid) {
        resolve();
      } else {
        reject("Invalid file type (must be JPG or PNG)");
      }
    }, 500); // Simulating delay
  });
}

interface DropzoneInit {
  instance: Dropzone;
  getValidatedFiles: () => File[];
  destroy: () => void;
}

/**
 * Initializes Dropzone independently. Manages its own list of validated files.
 * @param targetElementSelector CSS selector for the DOM element.
 * @returns An object with the instance and methods to interact, or null on failure.
 */
function dropzoneInit(targetElementSelector: string): DropzoneInit | null {
  validatedFiles = [];
  let currentInstance: Dropzone | null = null;

  const targetElement = document.querySelector<HTMLElement>(targetElementSelector);
  if (!targetElement) {
    console.error(`Dropzone target element "${targetElementSelector}" not found.`);
    return null;
  }
  if (!Dropzone) {
    console.error("Dropzone library not loaded.");
    return null;
  }

  Dropzone.autoDiscover = false;

  const dzOptions: Dropzone.DropzoneOptions = {
    url: "#",
    autoProcessQueue: false,
    acceptedFiles: "image/jpeg,image/png",
    addRemoveLinks: true,
    previewsContainer: document.getElementById("dz-preview-container") as HTMLElement,
    dictDefaultMessage: "",
    maxFiles: 30,
    init() {
      const dz = this as Dropzone;
      currentInstance = dz;

      dz.on("addedfile", (file: Dropzone.DropzoneFile) => {
        console.log(`Standalone Added: ${file.name}`);
        const statusEl = file.previewElement?.querySelector<HTMLElement>("[data-dz-status]");
        if (statusEl) statusEl.textContent = "Processing...";

        handleFileAsync(file)
          .then(() => {
            file.status = Dropzone.SUCCESS;
            const alreadyExists = validatedFiles.some(
              (f) => f.name === file.name && f.size === file.size
            );
            if (!alreadyExists) {
              validatedFiles.push(file);
              console.log(
                `Standalone Internal List: Added ${file.name}. Total: ${validatedFiles.length}`
              );
            }
            dz.emit("success", file, "Validation OK");
            if (statusEl) statusEl.textContent = "Ready";
          })
          .catch((err: any) => {
            file.status = Dropzone.ERROR;
            validatedFiles = validatedFiles.filter(
              (f) => !(f.name === file.name && f.size === file.size)
            );
            dz.emit("error", file, err?.message || err || "Processing failed");
            console.log(
              `Standalone Internal List: Validation failed for ${file.name}. Total: ${validatedFiles.length}`
            );
          })
          .finally(() => {
            dz.emit("complete", file);
          });
      });

      dz.on("removedfile", (file: Dropzone.DropzoneFile) => {
        validatedFiles = validatedFiles.filter(
          (f) =>
            !(f.name === file.name && f.size === file.size && f.lastModified === file.lastModified)
        );
        console.log(
          `Standalone Internal List: Removed ${file.name}. Total: ${validatedFiles.length}`
        );
      });
    },
  };

  try {
    const instance = new Dropzone(targetElement, dzOptions);
    console.log(`Standalone Dropzone initialized for "${targetElementSelector}"`);

    return {
      instance: instance,
      getValidatedFiles: () => [...validatedFiles],
      destroy: () => {
        if (instance && instance.element) {
          instance.destroy();
          console.log(`Standalone Dropzone for "${targetElementSelector}" destroyed.`);
        }
        validatedFiles = [];
        currentInstance = null;
      },
    };
  } catch (error) {
    console.error(
      `Failed to initialize standalone Dropzone for "${targetElementSelector}":`,
      error
    );
    return null;
  }
}

export { dropzoneInit, type DropzoneInit };
