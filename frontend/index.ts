import Alpine from "alpinejs";
import { TrainingModelFormClass } from "./lib/type/trainingModelForm";
import { DAL } from "./lib/api";
import { createBatches, createZip, ensureBoolean } from "./lib/utils";
import { ImageGenFormClass } from "./lib/type/ImageGenForm";

declare global {
  interface Window {
    __APP_ENV__: {
      apiBaseUrl: string;
      isLocal: boolean;
    };
    htmx: any;
  }
}

enum Stores {
  Image = "image",
  Toast = "toast",
  ImageGenForm = "imageGenForm",
  Uploader = "uploader",
  CreateModelForm = "createModelForm",
}

interface ToastStore {
  message: string;
  isVisible: boolean;
  isError: boolean;
  isWarn: boolean;
  success(message: string, duration?: number): this;
  warn(message: string, duration?: number): this;
  error(message: string, duration?: number): this;
}

Alpine.store(Stores.Toast, {
  isVisible: false,
  isWarn: false,
  isError: false,
  message: "",
  success(this: ToastStore, message: string, duration = 5000): ToastStore {
    this.isVisible = true;
    this.isError = false;
    this.message = message;
    setTimeout(() => (this.isVisible = false), duration);
    return this;
  },
  error(this: ToastStore, message: string, duration = 5000): ToastStore {
    this.isVisible = true;
    this.isError = true;
    this.message = message;
    setTimeout(() => (this.isVisible = false), duration);
    return this;
  },
  warn(this: ToastStore, message: string, duration = 5000): ToastStore {
    this.isVisible = true;
    this.isWarn = true;
    this.message = message;
    setTimeout(() => (this.isVisible = false), duration);
    return this;
  },
});

interface CreateModelFormStore extends TrainingModelFormClass {
  name: string;
  sex: string;
  age: number | string;
  ethnicity: string;
  eye_color: string;
  based_on: boolean;
  bald: boolean | string;
  consent: boolean;

  files: File[];
  zip: File | null;
  fileDragging: number | null;
  fileDropping: number | null;
  isSubmitting: boolean;
  previewUrls: string[];

  reset(): void;
  resetFiles(): void;
  submit(): Promise<void>;
  handleFileSelection(event: Event): void;
  addFiles(e: Event): void;
  humanFileSize(size: number): string;
  removeFile(index: number): void;
  reorderFilesOnDrop(e: DragEvent): void;
  dragEnterFile(e: DragEvent): void;
  dragStartFile(e: DragEvent): void;
  loadFilePreview(file: File): string | null;
  revokePreviewUrls(): void;
  handleFileDrop(e: DragEvent): void;
}

Alpine.store(Stores.CreateModelForm, {
  // Form fields
  name: "",
  sex: "",
  age: "" as string | number,
  ethnicity: "",
  eye_color: "",
  based_on: true,
  bald: "",
  consent: false,

  // File handling
  files: [] as File[],
  zip: null as File | null,
  fileDragging: null as number | null,
  fileDropping: null as number | null,
  previewUrls: [] as string[],

  isSubmitting: false,

  // Reset form
  reset(this: CreateModelFormStore): void {
    this.name = "";
    this.sex = "";
    this.age = "" as string | number;
    this.ethnicity = "";
    this.eye_color = "";
    this.bald = "";
    this.based_on = true;
    this.consent = false;
    this.zip = null;
    this.isSubmitting = false;
    this.resetFiles();
  },

  // Form submission
  async submit(this: CreateModelFormStore): Promise<void> {
    if (this.isSubmitting) {
      return;
    }
    if (!this.files.length || this.files.length < 1) {
      Alpine.store(Stores.Toast).error("No files selected.");
      return;
    }
    if (!this.files.length || this.files.length < 10) {
      Alpine.store(Stores.Toast).error(
        `You still need to add ${10 - this.files.length} more image(s)`
      );
      return;
    }
    if (Number(this.age) < 18 || Number(this.age) > 120) {
      return;
    }

    this.isSubmitting = true;

    try {
      // Create zip
      this.zip = await createZip(this.files, this.name);

      const modelData = TrainingModelFormClass.create({
        name: this.name,
        sex: this.sex,
        age: this.age ?? 18,
        based_on: this.based_on,
        ethnicity: this.ethnicity,
        eye_color: this.eye_color,
        bald: ensureBoolean(this.bald),
        consent: this.consent,
        file_type: TrainingModelFormClass.file_type,
      });

      // Create and upload to S3
      await DAL.Complete.S3UploadTrainingModel.saveToS3(modelData, this.zip);
      Alpine.store(Stores.Toast).success("New Model Successfully Created");

      // Get training models html
      await DAL.Complete.Htmx.getTrainingModels();

      this.reset();
    } catch (error) {
      Alpine.store(Stores.Toast).error("error");
      this.isSubmitting = false;
    }
  },

  // File handling methods
  handleFileSelection(this: CreateModelFormStore, event: Event): void {
    const input = event.target as HTMLInputElement;
    if (!input.files?.length) return;

    // Check all files for duplicates
    const newFiles = Array.from(input.files).filter(
      (newFile) =>
        !this.files.some(
          (existingFile) =>
            existingFile.name === newFile.name &&
            existingFile.size === newFile.size &&
            existingFile.lastModified === newFile.lastModified
        )
    );

    if (newFiles.length < input.files.length) {
      Alpine.store("toast").warn(`Skipped ${input.files.length - newFiles.length} duplicate files`);
    }

    if (newFiles.length) {
      this.files = newFiles; // Replace existing files
    } else if (input.files.length) {
      Alpine.store("toast").warn("All selected files are duplicates");
    }
  },

  addFiles(this: CreateModelFormStore, e: Event): void {
    const input = e.target as HTMLInputElement;
    if (!input.files?.length) return;

    // Check all files for duplicates
    const newFiles = Array.from(input.files).filter(
      (newFile) =>
        !this.files.some(
          (existingFile) =>
            existingFile.name === newFile.name &&
            existingFile.size === newFile.size &&
            existingFile.lastModified === newFile.lastModified
        )
    );

    if (newFiles.length < input.files.length) {
      Alpine.store("toast").error(
        `Skipped ${input.files.length - newFiles.length} duplicate files`
      );
    }

    if (newFiles.length) {
      this.files = [...this.files, ...newFiles];
      input.value = "";
    } else if (input.files.length) {
      Alpine.store("toast").error("All selected files are duplicates");
    }
  },

  humanFileSize(this: CreateModelFormStore, size: number): string {
    if (size === 0) return "0 B";
    const units = ["B", "kB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(size) / Math.log(1024));
    const num = size / Math.pow(1024, i);
    return `${num.toFixed(2)} ${units[i]}`;
  },

  removeFile(this: CreateModelFormStore, index: number): void {
    this.files.splice(index, 1);
  },

  reorderFilesOnDrop(this: CreateModelFormStore, e: DragEvent): void {
    e.preventDefault();
    if (this.fileDragging !== null && this.fileDropping !== null) {
      const removed = this.files.splice(this.fileDragging, 1);
      this.files.splice(this.fileDropping, 0, ...removed);
    }
    this.fileDragging = null;
    this.fileDropping = null;
  },

  dragEnterFile(this: CreateModelFormStore, e: DragEvent): void {
    const target = (e.target as HTMLElement).closest("[draggable]");
    if (target) {
      this.fileDropping = Number(target.getAttribute("data-index"));
    }
  },

  dragStartFile(this: CreateModelFormStore, e: DragEvent): void {
    const target = (e.target as HTMLElement).closest("[draggable]");
    if (target) {
      this.fileDragging = Number(target.getAttribute("data-index"));
      e.dataTransfer!.effectAllowed = "move";
    }
  },
  // To avoid memory leaks
  loadFilePreview(this: CreateModelFormStore, file: File): string | null {
    if (file.type.startsWith("image/") || file.type.startsWith("video/")) {
      const url = URL.createObjectURL(file);
      this.previewUrls.push(url);
      return url;
    }
    return null;
  },
  revokePreviewUrls(this: CreateModelFormStore) {
    this.previewUrls.forEach((url) => URL.revokeObjectURL(url));
    this.previewUrls = [];
  },
  resetFiles(this: CreateModelFormStore): void {
    this.files = [];
    this.fileDragging = null;
    this.fileDropping = null;
    this.revokePreviewUrls();
  },
  handleFileDrop(this: CreateModelFormStore, e: DragEvent): void {
    e.preventDefault();
    e.stopPropagation();

    if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
      const droppedFiles = Array.from(e.dataTransfer.files);

      // Filter for allowed types
      const imageFiles = droppedFiles.filter(
        (file) => file.type === "image/png" || file.type === "image/jpeg"
      );

      if (imageFiles.length < droppedFiles.length) {
        Alpine.store(Stores.Toast).error(
          `Skipped ${droppedFiles.length - imageFiles.length} non-image files.`
        );
      }

      // Check for duplicates against existing files
      const newFiles = imageFiles.filter(
        (newFile) =>
          !this.files.some(
            (existingFile) =>
              existingFile.name === newFile.name &&
              existingFile.size === newFile.size &&
              existingFile.lastModified === newFile.lastModified
          )
      );

      if (newFiles.length < imageFiles.length) {
        Alpine.store(Stores.Toast).warn(
          `Skipped ${imageFiles.length - newFiles.length} duplicate files`
        );
      }

      // Check if there are actually new files to add
      if (newFiles.length > 0) {
        const originalLength = this.files.length;
        this.files = [...this.files, ...newFiles];
      } else if (imageFiles.length > 0) {
        Alpine.store(Stores.Toast).warn("All dropped files are duplicates or not images.");
      }
    }

    // Reset dragging state
    this.fileDragging = null;
    this.fileDropping = null;
  },
});

// Define the structure of the store's state and methods
interface ImageGenFormStore {
  advancedOptionsOpen: boolean;
  inferenceSteps: number;
  numImages: number;
  isLoading: boolean;
  selectedModelId: string;

  init(): void;
  reset(): void;
  toggleAdvancedOptions(): void;
  decrementImages(): void;
  incrementImages(maxImages?: number): void;
  handleCreateRequest(event: SubmitEvent): Promise<void>;
}

Alpine.store(Stores.ImageGenForm, {
  advancedOptionsOpen: false,
  inferenceSteps: 28,
  numImages: 8,
  isLoading: false,
  selectedModelId: "",

  init() {},
  reset(this: ImageGenFormStore): void {
    (this.advancedOptionsOpen = false), (this.isLoading = false);
  },

  toggleAdvancedOptions(this: ImageGenFormStore) {
    this.advancedOptionsOpen = !this.advancedOptionsOpen;
  },

  decrementImages(this: ImageGenFormStore) {
    this.numImages = Math.max(1, this.numImages - 1);
  },

  incrementImages(this: ImageGenFormStore, maxImages: number = 20) {
    this.numImages = Math.min(maxImages, this.numImages + 1);
  },

  async handleCreateRequest(this: ImageGenFormStore, event: SubmitEvent): Promise<void> {
    if (this.isLoading) return; // Prevent multiple submissions

    this.advancedOptionsOpen = false;

    const form = event.target as HTMLFormElement;
    if (!form) {
      console.error("Form element not found in event");
      return;
    }

    const formData = new FormData(form);

    // Construct payload with controlled inputs
    const payload = {
      prompt: (formData.get("prompt") as string)?.trim() || "",
      training_model_id: isNaN(Number(formData.get("training_model_id")))
        ? formData.get("training_model_id")
        : Number(formData.get("training_model_id")),
      num_inference_steps: this.inferenceSteps,
      num_images: this.numImages,
      image_size: (formData.get("image_size") as string) || "1024x1024",
    };

    // Validation
    if (!payload.prompt) {
      Alpine.store(Stores.Toast).error("Please enter a prompt description.");
      return; // Return here to stop further execution
    }

    const modelData = ImageGenFormClass.create(payload);
    console.log("Submitting payload:", modelData);

    this.isLoading = true;
    Alpine.store(Stores.Toast).success("Image generation started!");

    try {
      const [batches, singles] = createBatches(modelData.num_images);
      console.log(batches, singles);

      modelData.num_images = 4;
      for (let i = 0; i < batches; i++) {
        console.log(`Batches ${i + 1} of ${batches} send`);
        // await sendRequest(url, modelData);
        await DAL.Complete.Htmx.imageGenerationHtmx(modelData, "drive-gallery", "afterbegin");

        // Set time out for 2 seconds
        await DAL.delay(2000);
      }

      modelData.num_images = 1;
      for (let i = 0; i < singles; i++) {
        console.log(`Singles ${i + 1} of ${singles} send`);
        // await sendRequest(url, modelData);
        await DAL.Complete.Htmx.imageGenerationHtmx(modelData, "drive-gallery", "afterbegin");

        // Set time out for 1 seconds
        await DAL.delay(500);
      }

      // await new Promise((resolve) => setTimeout(resolve, 3000));
      console.log("Processing completed.");
      this.reset();
    } catch (error) {
      console.error(error);
      Alpine.store(Stores.Toast).error("An unexpected error occurred. Please try again.");
    } finally {
      this.isLoading = false; // Ensure loading state is reset
    }
  },
});

// interface UploaderStore {
//   uploadImageFromUrlToS3(imageUrl: string, presignedUrl: string, notifyBackendUrl: string): void;
// }

// // Register Alpine store
// Alpine.store(Stores.Uploader, {
//   uploadImageFromUrlToS3: DAL.uploadImageFromUrlToS3,
// });

// You might need to explicitly tell TypeScript about the store on the Alpine object
// if you haven't extended the Alpine interface globally.
declare module "alpinejs" {
  interface Stores {
    toast: ToastStore;
    imageGenForm: ImageGenFormStore;
    // uploader: UploaderStore;
  }
}

async function oAuth2(provider_link: string) {
  try {
    const response = await fetch(`${provider_link}`);
    if (!response.ok) {
      throw new Error("Failed to get OAuth URL");
    }

    const oauthLink = await response.text();
    window.location.href = oauthLink;
  } catch (err) {
    console.error("OAuth2 error:", err);
    alert("Could not start OAuth2 login. Please try again.");
  }
}

(window as any).oAuth2 = oAuth2;

Alpine.start();
