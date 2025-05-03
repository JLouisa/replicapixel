import axios from "redaxios";
import { parse } from "valibot";
import type { Options } from "redaxios";
import { PreSignUrlRequestClass, type PreSignUrlRequest } from "./type/presignedUrlRequest_type";
import { ServerResponseClass, ServerResponseSchema, type ServerResponse } from "./type/response";
import type { TrainingModelFormClass } from "./type/trainingModelForm";
import type { ImageGenForm } from "./type/ImageGenForm";
import Alpine from "alpinejs";

const DEFAULT_BASE_URL = window.__APP_ENV__.apiBaseUrl;

enum Api {
  Upload = "/api/models",
  Image = "/api/images",
  Dashboard = "/dashboard",
}

enum AddOnUrl {
  One = "/one",
  Many = "/many",
  All = "/all",
}

const BackendUrl = {
  Upload: {
    Base: Api.Upload,
    UploadPostZip: Api.Upload + "/zip",
    UploadPostMany: Api.Upload + AddOnUrl.Many,
    UploadPreSignedUrl: Api.Upload + "/pre-signed-url",
    UploadPreSignedUrlTraining: Api.Upload + "/upload",
    UploadOneCompleted: Api.Upload + "/upload/completed",
  },
  Image: {
    Base: Api.Image,
    GenerateImage: Api.Image + "/generate/test",
  },
  Dashboard: {
    Base: Api.Dashboard,
    trainingModelPartial: Api.Dashboard + "/partial/models",
    trainingModel: Api.Dashboard + "/models",
  },
};

const axiosInstance = axios.create({
  baseURL: DEFAULT_BASE_URL,
  withCredentials: true, // Include credentials (cookies) in requests
  headers: {
    "Content-Type": "application/json",
  },
});

const backendApi = {
  async get<T>(url: string, config?: Options): Promise<T> {
    try {
      const response = await axiosInstance.get<T>(url, config);
      console.log(response.data);
      const serverResponse = new ServerResponseClass<T>(response.data);
      return serverResponse.getData();
    } catch (error) {
      console.error("GET request failed:", error);
      throw error;
    }
  },
  async post<T>(url: string, body: any, config?: Options): Promise<T> {
    try {
      const response = await axiosInstance.post(url, body, config);
      console.log("response login Data", response.data);
      const serverResponse = new ServerResponseClass<T>(response.data);
      return serverResponse.getData();
    } catch (error) {
      console.error("POST request failed:", error);
      //   toast.error(`Oops! Something went wrong. Try uploading again! Error: ${error}`);
      throw error;
    }
  },
  async patch<T>(url: string, config?: Options): Promise<T> {
    try {
      const response = await axiosInstance.patch(url, null, config);
      console.log(response.data);
      const serverResponse = new ServerResponseClass<T>(response.data);
      return serverResponse.getData();
    } catch (error) {
      console.error("PATCH request failed:", error);
      throw error;
    }
  },
  async put<T>(url: string, body: any, config?: Options): Promise<T> {
    try {
      const response = await axiosInstance.put(url, body, config);
      console.log(response.data);
      const serverResponse = new ServerResponseClass<T>(response.data);
      return serverResponse.getData();
    } catch (error) {
      console.error("PUT request failed:", error);
      throw error;
    }
  },
  async delete<T>(url: string, config?: Options): Promise<T> {
    try {
      const response = await axiosInstance.delete(url, config);
      console.log(response.data);
      const serverResponse = new ServerResponseClass<T>(response.data);
      return serverResponse.getData();
    } catch (error) {
      console.error("DELETE request failed:", error);
      throw error;
    }
  },
  async SimpleGet(url: string, config?: Options) {
    try {
      const response = await axiosInstance.get(url, config);
      console.log(response.data.data);
      return response.data.data;
    } catch (error) {
      console.error("SimpleGet failed:", error);
      //   toast.error(`Oops! Something went wrong. Error: ${error}`);
      throw error;
    }
  },
  async SimplePost(url: string, body: any, config?: Options) {
    try {
      const response = await axiosInstance.post(url, body, config);
      console.log(response.data);
      const result: ServerResponse = parse(ServerResponseSchema, response.data);

      return result;
    } catch (error) {
      console.error("SimplePost request failed:", error);
      //   toast.error(`Oops! Something went wrong. Error: ${error}`);
      throw error;
    }
  },
  async SimpleUpdate(url: string, body: any, config?: Options) {
    try {
      const response = await axiosInstance.put(url, body, config);
      console.log(response.data);
      const result: ServerResponse = parse(ServerResponseSchema, response.data);
      return result;
    } catch (error) {
      console.error("SimpleUpdate request failed:", error);
      //   toast.error(`Oops! Something went wrong. Try uploading again! Error: ${error}`);
      throw error;
    }
  },
  async SimplePatch(url: string, config?: Options) {
    try {
      const response = await axiosInstance.patch(url, config);
      console.log(response.data);
      const result: ServerResponse = parse(ServerResponseSchema, response.data);
      return result;
    } catch (error) {
      console.error("SimplePatch request failed:", error);
      throw error;
    }
  },
  async AnyPatch(url: string, config?: Options) {
    try {
      const response = await axiosInstance.patch(url, config);
      console.log(response.data);
      return response.data;
    } catch (error) {
      console.error("SimplePatch request failed:", error);
      throw error;
    }
  },
  async SimpleDelete(url: string, config?: Options) {
    try {
      const response = await axiosInstance.delete(url, config);
      console.log(response.data);
      const result: ServerResponse = parse(ServerResponseSchema, response.data);
      return result;
    } catch (error) {
      console.error("SimpleDelete request failed:", error);
      throw error;
    }
  },
  async putS3(info: PreSignUrlRequest, payload: File, config?: Options) {
    const axiosS3Instance = axios.create({
      baseURL: DEFAULT_BASE_URL,
      headers: {
        "Content-Type": info.file_type,
      },
    });
    try {
      const response = await axiosS3Instance.put(info.pre_url as string, payload, config);
      console.log(response);
      console.log(response.data);
      return response.data;
    } catch (error) {
      console.error("POST request failed:", error);
      throw error;
    }
  },
  async getHTMX(url: string, target: string, swapStyle: string, config?: Options) {
    try {
      const response = await axiosInstance.get<string>(url, config);

      // --- Manual OOB Processing ---
      // 1. Create a temporary, disconnected container for the response HTML
      const tempContainer = document.createElement("div");
      tempContainer.innerHTML = response.data;

      // 2. Find all OOB elements within the temporary container
      const oobElements = tempContainer.querySelectorAll("[hx-swap-oob]");

      // 3. Process each OOB element
      oobElements.forEach((oobElement) => {
        const oobAttrValue = oobElement.getAttribute("hx-swap-oob");
        let oobSwapStyle = "outerHTML"; // Default swap style for OOB
        let oobTargetSelector = "#" + oobElement.id; // Default target is the element's own ID

        // Check for custom OOB swap styles/selectors if needed
        // Basic parsing example (can be expanded based on htmx source logic)
        if (oobAttrValue && oobAttrValue !== "true") {
          if (oobAttrValue.includes(":")) {
            const parts = oobAttrValue.split(":", 2);
            oobSwapStyle = parts[0] || oobSwapStyle;
            oobTargetSelector = parts[1] || oobTargetSelector;
          } else {
            oobSwapStyle = oobAttrValue; // Only swap style was specified
          }
        }

        // Find the actual target(s) in the main document
        const oobTargets = document.querySelectorAll(oobTargetSelector);

        if (oobTargets.length > 0) {
          // Remove the OOB attribute before swapping to avoid issues
          // Note: We swap the element itself from the temp container
          oobElement.removeAttribute("hx-swap-oob");

          oobTargets.forEach((target) => {
            // Use htmx.swap for each OOB target
            // Pass the OOB element's outerHTML as the content
            window.htmx.swap(target, oobElement.outerHTML, { swapStyle: oobSwapStyle });
          });

          // Remove the OOB element from the temp container so it's not part of the main swap
          oobElement.remove();
        } else {
          // Target not found, log an error or warning
          console.warn(`Target not found for selector: ${oobTargetSelector}`);
          // Still remove it from the temp container
          oobElement.remove();
        }
      });

      // --- Main Swap ---

      // 4. Find the main target element *after* OOB processing
      const mainTargetElement = document.getElementById(target);
      if (!mainTargetElement) {
        console.error(`Target element ${target} not found!`);
        DAL.Toast.error("An unexpected error occurred (gallery missing). Refresh the page.");
        return; // Exit if the target doesn't exist
      }

      // 5. Get the remaining HTML from the temp container (OOB elements are gone)
      const remainingContent = tempContainer.innerHTML;

      // 6. Perform the main swap with the remaining content
      window.htmx.swap(mainTargetElement, remainingContent, { swapStyle });

      // Optional: Process scripts if htmx.swap doesn't automatically (it usually does)
      // window.htmx.process(mainTargetElement); // Might be needed if the swapped content has hx-* attributes
    } catch (error) {
      console.error("Fetch request failed:", error);
      DAL.Toast.error("An unexpected error occurred. Please try again.");
      throw error;
    }
  },
  async postHTMX(
    url: string,
    payload: ImageGenForm,
    target: string,
    swapStyle: string,
    config?: Options
  ) {
    try {
      const response = await axiosInstance.post<string>(url, payload, config);

      // --- Manual OOB Processing ---
      // 1. Create a temporary, disconnected container for the response HTML
      const tempContainer = document.createElement("div");
      tempContainer.innerHTML = response.data;

      // 2. Find all OOB elements within the temporary container
      const oobElements = tempContainer.querySelectorAll("[hx-swap-oob]");

      // 3. Process each OOB element
      oobElements.forEach((oobElement) => {
        const oobAttrValue = oobElement.getAttribute("hx-swap-oob");
        let oobSwapStyle = "outerHTML"; // Default swap style for OOB
        let oobTargetSelector = "#" + oobElement.id; // Default target is the element's own ID

        // Check for custom OOB swap styles/selectors if needed
        // Basic parsing example (can be expanded based on htmx source logic)
        if (oobAttrValue && oobAttrValue !== "true") {
          if (oobAttrValue.includes(":")) {
            const parts = oobAttrValue.split(":", 2);
            oobSwapStyle = parts[0] || oobSwapStyle;
            oobTargetSelector = parts[1] || oobTargetSelector;
          } else {
            oobSwapStyle = oobAttrValue; // Only swap style was specified
          }
        }

        // Find the actual target(s) in the main document
        const oobTargets = document.querySelectorAll(oobTargetSelector);

        if (oobTargets.length > 0) {
          // Remove the OOB attribute before swapping to avoid issues
          // Note: We swap the element itself from the temp container
          oobElement.removeAttribute("hx-swap-oob");

          oobTargets.forEach((target) => {
            // Use htmx.swap for each OOB target
            // Pass the OOB element's outerHTML as the content
            window.htmx.swap(target, oobElement.outerHTML, { swapStyle: oobSwapStyle });
          });

          // Remove the OOB element from the temp container so it's not part of the main swap
          oobElement.remove();
        } else {
          // Target not found, log an error or warning
          console.warn(`Target not found for selector: ${oobTargetSelector}`);
          // Still remove it from the temp container
          oobElement.remove();
        }
      });

      // --- Main Swap ---

      // 4. Find the main target element *after* OOB processing
      const mainTargetElement = document.getElementById(target);
      if (!mainTargetElement) {
        console.error(`Target element ${target} not found!`);
        DAL.Toast.error("An unexpected error occurred (gallery missing). Refresh the page.");
        return; // Exit if the target doesn't exist
      }

      // 5. Get the remaining HTML from the temp container (OOB elements are gone)
      const remainingContent = tempContainer.innerHTML;

      // 6. Perform the main swap with the remaining content
      window.htmx.swap(mainTargetElement, remainingContent, { swapStyle });

      // Optional: Process scripts if htmx.swap doesn't automatically (it usually does)
      // window.htmx.process(mainTargetElement); // Might be needed if the swapped content has hx-* attributes
    } catch (error) {
      console.error("Fetch request failed:", error);
      DAL.Toast.error("An unexpected error occurred. Please try again.");
      throw error;
    }
  },
};

export const DAL = {
  Backend: {
    htmx: {
      async getHtmx(url: string, target: string, swapStyle: string) {
        try {
          return await backendApi.getHTMX(url, target, swapStyle);
        } catch (error) {
          return DAL.handleError("Something went wrong uploading image", error, false);
        }
      },
      ImageGeneration: {
        async generateImage(payload: ImageGenForm, target: string, swapStyle: string) {
          console.log(BackendUrl.Image.GenerateImage);
          try {
            return await backendApi.postHTMX(
              BackendUrl.Image.GenerateImage,
              payload,
              target,
              swapStyle
            );
          } catch (error) {
            return DAL.handleError(
              "Something went wrong generating image. Code: 1000",
              error,
              false
            );
          }
        },
      },
    },
    UploadService: {
      async getPreSignUrlForUpload(payload: PreSignUrlRequest) {
        try {
          return await backendApi.post<PreSignUrlRequestClass>(BackendUrl.Upload.Base, payload);
        } catch (error) {
          return DAL.handleError("Something went wrong uploading image. Code: 1000", error, false);
        }
      },
      async getPreSignUrlForUploadTraining(payload: TrainingModelFormClass) {
        try {
          return await backendApi.post<PreSignUrlRequestClass>(
            BackendUrl.Upload.UploadPreSignedUrlTraining,
            payload
          );
        } catch (error) {
          return DAL.handleError("Something went wrong uploading image. Code: 1000", error, false);
        }
      },
      async markUploadSuccess(request: PreSignUrlRequest) {
        const url = BackendUrl.Upload.UploadOneCompleted + "/" + request.id;
        try {
          return await backendApi.AnyPatch(url);
        } catch (error) {
          throw error;
        }
      },
    },
  },
  Integrations: {
    AwsS3: {
      async uploadToS3(request: PreSignUrlRequest, payload: File) {
        try {
          return await backendApi.putS3(request, payload);
        } catch (error) {
          return DAL.handleError("Something went wrong uploading image", error, false);
        }
      },
    },
  },
  Complete: {
    Htmx: {
      async imageGenerationHtmx(payload: ImageGenForm, target: string, swapStyle: string) {
        try {
          return await DAL.Backend.htmx.ImageGeneration.generateImage(payload, target, swapStyle);
        } catch (error) {
          return DAL.handleError("Something went wrong uploading image", error, false);
        }
      },
      async getTrainingModels() {
        const url = BackendUrl.Dashboard.trainingModelPartial;
        const target = "#dashboard_content";
        const swap = "innerHTML";
        try {
          return await window.htmx.ajax("GET", url, { target, swap }).then((_: any) => {
            window.history.pushState({}, "", BackendUrl.Dashboard.trainingModel);
          });
        } catch (error) {
          return DAL.handleError("Something went wrong uploading image", error, false);
        }
      },
    },
    // S3Upload: {
    //   async saveToS3(file: File) {
    //     try {
    //       const preSignRequest = PreSignUrlRequestClass.newToServer(file.name, file.type);
    //       // Step 1: Obtain presigned URL from the server
    //       const presignedResponse = await DAL.Backend.UploadService.getPreSignUrlForUpload(
    //         preSignRequest
    //       );

    //       // Step 2: Upload the file to S3 using the presigned URL
    //       await DAL.Integrations.AwsS3.uploadToS3(presignedResponse, file);

    //       // Step 3: Notify the server that the upload was successful
    //       const item = await DAL.Backend.UploadService.markUploadSuccess(presignedResponse);

    //       // Step 4: Notify the user that everything is complete
    //       // toast.success("Image successfully uploaded!");
    //       return item;
    //     } catch (error) {
    //       return DAL.handleError("Failed to upload the image. Please try again.", error, false);
    //     }
    //   },
    // },
    S3UploadTrainingModel: {
      async saveToS3(modelData: TrainingModelFormClass, file: File) {
        try {
          const preSignRequest = PreSignUrlRequestClass.newToServer(file.name, file.type);
          console.log("preSignRequest SaveToS3: ", preSignRequest);

          // Step 1: Obtain presigned URL from the server
          const presignedResponse = await DAL.Backend.UploadService.getPreSignUrlForUploadTraining(
            modelData
          );
          // Step 2: Upload the file to S3 using the presigned URL
          console.log("presignedResponse SaveToS3: ", presignedResponse);
          await DAL.Integrations.AwsS3.uploadToS3(presignedResponse, file);

          // Step 3: Notify the server that the upload was successful
          const item = await DAL.Backend.UploadService.markUploadSuccess(presignedResponse);

          // Step 4: Notify the user that everything is complete
          DAL.Toast.success("Model successfully uploaded!");
          return item;
        } catch (error) {
          throw error;
        }
      },
    },
  },
  // Function to introduce a delay
  delay: (ms: number) => {
    console.log(`Pausing for ${ms / 1000} seconds...`);
    return new Promise((resolve) => setTimeout(resolve, ms));
  },
  // Helper functions
  Toast: {
    success(message: string) {
      Alpine.store("toast").success(message);
    },
    error(message: string) {
      Alpine.store("toast").error(message);
    },
  },
  Console(message: string) {
    console.log(message);
  },
  // Generic error handling function to reduce duplication
  handleError(action: string, error: any, useToast: boolean) {
    const errorMessage = `Something went wrong ${action}: ${error.message || error}`;
    console.error("Error uploading file:", error);
    Alpine.store("toast").success(errorMessage);
    throw new Error(errorMessage);
  },
  async uploadImageFromUrlToS3(imageUrl: string, presignedUrl: string, notifyBackendUrl: string) {
    try {
      const imageResponse = await fetch(imageUrl);
      const blob = await imageResponse.blob();

      const s3Upload = await fetch(presignedUrl, {
        method: "PUT",
        body: blob,
        headers: {
          "Content-Type": blob.type,
        },
      });

      if (!s3Upload.ok) throw new Error("Upload to S3 failed");

      await fetch(notifyBackendUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ uploaded: true }),
      });

      console.log("✅ Upload complete");
    } catch (err) {
      console.error("❌ Upload failed", err);
    }
  },
};
