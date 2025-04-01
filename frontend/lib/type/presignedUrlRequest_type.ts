import { object, optional, pipe, safeParse, string, url, uuid, type InferInput } from "valibot";

export const PreSignUrlRequestSchema = object({
  id: optional(pipe(string(), uuid())),
  name: string(),
  file_type: string(),
  pre_url: optional(pipe(string(), url())),
});
export type PreSignUrlRequest = InferInput<typeof PreSignUrlRequestSchema>;

export class PreSignUrlRequestClass implements PreSignUrlRequest {
  id: string | undefined;
  name: string;
  file_type: string;
  pre_url: string | undefined;

  constructor(
    id: string | undefined,
    name: string,
    file_type: string,
    pre_url: string | undefined
  ) {
    this.id = id;
    this.name = name;
    this.file_type = file_type;
    this.pre_url = pre_url;
  }

  private static vali(data: any): PreSignUrlRequest {
    const vali = safeParse(PreSignUrlRequestSchema, data);
    if (vali.success) {
      return vali.output;
    } else {
      const errorDetails = vali.issues;
      // .map((err) => `Field: ${err.path.join(".")}, Error: ${err.message}`)
      // .join(", ");
      //   toast.error("Oops! Something went wrong. Try Uploading again!");
      throw new Error(`PreSignUrlRequest Parsing Failed: ${errorDetails}`);
    }
  }

  static newToServer(
    name: string,
    file_type: string,
    id: string | undefined = undefined,
    pre_url: string | undefined = undefined
  ): PreSignUrlRequestClass {
    const form = { id, name, file_type, pre_url };

    try {
      const newPreSignUrl = PreSignUrlRequestClass.vali(form);

      // Always return a new instance, regardless of undefined fields
      const { id, name, file_type, pre_url } = newPreSignUrl;

      return new PreSignUrlRequestClass(id, name, file_type, pre_url);
    } catch (error) {
      console.error(error);
      //   toast.error("Oops! Something went wrong. Try Uploading again!");
      throw error; // Ensure no undefined is returned
    }
  }

  static newFromServer(data: any): PreSignUrlRequestClass {
    try {
      const newPreSignUrl = PreSignUrlRequestClass.vali(data);

      // No conditional check for undefined, always return the instance
      const { id, name, file_type, pre_url } = newPreSignUrl;

      return new PreSignUrlRequestClass(id, name, file_type, pre_url);
    } catch (error) {
      console.error(error);
      //   toast.error("Oops! Something went wrong. Try Uploading again!");
      throw error; // Ensure an error is thrown, not undefined
    }
  }

  static create(
    name: string,
    file_type: string,
    id: string | undefined = undefined,
    pre_url: string | undefined = undefined
  ): PreSignUrlRequest {
    return new PreSignUrlRequestClass(id, name, file_type, pre_url);
  }
}
