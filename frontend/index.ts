import Alpine from "alpinejs";
import { TrainingModelFormClass } from "./lib/type/trainingModelForm";
import { DAL } from "./lib/api";
import { createBatches, createZip, ensureBoolean } from "./lib/utils";
import { ImageGenFormClass, type ImageGenForm } from "./lib/type/ImageGenForm";

declare global {
  interface Window {
    __APP_ENV__: {
      apiBaseUrl: string;
      isLocal: boolean;
    };
    htmx: any;
  }
}
console.log("Index Environment: ", window.__APP_ENV__.apiBaseUrl);

enum Stores {
  Image = "image",
  Toast = "toast",
  ImageGenForm = "imageGenForm",
  Uploader = "uploader",
}

interface CreateModelFormStore extends TrainingModelFormClass {
  files: File[];
  zip: File | null;
  image_zip_url: string;

  reset(): void;
  submit(): Promise<void>;
  handleFileSelection(event: Event): void;
}

Alpine.store("createModelForm", {
  name: "",
  sex: "",
  age: 18,
  ethnicity: "",
  eye_color: "",
  creative: 1000,
  based_on: true,
  bald: false,
  consent: false,
  files: [] as File[],
  zip: null as File | null,
  image_zip_url: "",

  reset(this: CreateModelFormStore): void {
    this.name = "";
    this.sex = "";
    this.age = 18;
    this.ethnicity = "";
    this.eye_color = "";
    this.creative = 1000;
    this.bald = false;
    this.based_on = true;
    this.consent = false;
    this.files = [];
    this.zip = null;
    this.image_zip_url = "";
  },

  async submit(this: CreateModelFormStore): Promise<void> {
    if (!this.files.length || this.files.length < 2) {
      console.error("No files selected.");
      return;
    }

    // Create zip
    this.zip = await createZip(this.files, this.name);

    // Ensure `TrainingModelForm.create()` and `APIClient` exist
    const modelData = TrainingModelFormClass.create({
      name: this.name,
      sex: this.sex,
      age: this.age ?? 18,
      based_on: this.based_on,
      ethnicity: this.ethnicity,
      eye_color: this.eye_color,
      creative: this.creative ?? 40,
      bald: ensureBoolean(this.bald),
      consent: this.consent,
      file_type: TrainingModelFormClass.file_type,
    });

    console.log("Model Data:", modelData);

    // Create and upload to S3
    const item = await DAL.Complete.S3UploadTrainingModel.saveToS3(modelData, this.zip);
    console.log("Success: ", item);

    await DAL.Complete.Htmx.getTrainingModels();

    this.reset();
  },

  handleFileSelection(this: CreateModelFormStore, event: Event): void {
    const input = event.target as HTMLInputElement;
    if (input.files) {
      this.files = Array.from(input.files);
    }
  },
});

interface ToastStore {
  message: string;
  isVisible: boolean;
  isError: boolean;
  success(message: string, duration?: number): this;
  error(message: string, duration?: number): this;
}

Alpine.store(Stores.Toast, {
  isVisible: false,
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
});

// Define the structure of the store's state and methods
interface ImageGenFormStore {
  advancedOptionsOpen: boolean;
  inferenceSteps: number;
  numImages: number;
  isLoading: boolean;
  selectedModelId: string;

  // Methods
  init(): void;
  reset(): void;
  toggleAdvancedOptions(): void;
  decrementImages(): void;
  incrementImages(maxImages?: number): void;
  handleCreateRequest(event: SubmitEvent): Promise<void>;
}

// Register the store with Alpine
Alpine.store(Stores.ImageGenForm, {
  advancedOptionsOpen: false,
  inferenceSteps: 28,
  numImages: 8,
  isLoading: false,
  selectedModelId: "",

  // --- Methods ---
  init() {
    console.log("ImageGenForm store initialized");
  },
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

async function oAuth2(provider: string) {
  try {
    const response = await fetch(`/api/oauth2/${provider}`);
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
