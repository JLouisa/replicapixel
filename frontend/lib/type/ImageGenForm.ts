import { object, number, string, type InferInput } from "valibot";
import { safeParse, union, pipe, uuid, null as null_ } from "valibot";

// 1. Schema Definition
export const ImageGenFormSchema = object({
  prompt: string(),
  training_model_pid: union([pipe(string(), uuid()), null_()]),
  num_inference_steps: number(),
  num_images: number(),
  image_size: string(),
});

// 2. Type derived from schema
export type ImageGenForm = InferInput<typeof ImageGenFormSchema>;

// 3. Class Implementation (minimal)
export class ImageGenFormClass implements ImageGenForm {
  // Static defaults (optional)
  static num_inference_steps: 4000;
  static num_images: 8;

  constructor(
    public prompt: string,
    public training_model_pid: string | null,
    public num_inference_steps: number = this.num_inference_steps,
    public num_images: number = this.num_images,
    public image_size: string
  ) {}

  // Static constructor with validation
  static create(data: unknown): ImageGenFormClass {
    const result = safeParse(ImageGenFormSchema, data);
    if (!result.success) {
      throw new Error("Invalid image generation data");
    }
    return new ImageGenFormClass(
      result.output.prompt,
      result.output.training_model_pid,
      result.output.num_inference_steps,
      result.output.num_images,
      result.output.image_size
    );
  }

  // Static initializer (optional)
}
