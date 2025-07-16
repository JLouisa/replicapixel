import { object, string, boolean, union, null_, safeParse, type InferOutput } from "valibot";

export const VideoGenFormSchema = object({
  name: string(),
  prompt: string(),
  negative_prompt: union([string(), null_()]),
  aspect_ratio: string(),
  duration: string(),
  enhance_prompt: boolean(),
  generate_audio: boolean(),
});
export type VideoGenForm = InferOutput<typeof VideoGenFormSchema>;

export class VideoGenFormClass implements VideoGenForm {
  constructor(
    public name: string,
    public prompt: string,
    public negative_prompt: string | null,
    public aspect_ratio: string,
    public duration: string,
    public enhance_prompt: boolean,
    public generate_audio: boolean
  ) {}
  static create(data: unknown): VideoGenFormClass {
    const result = safeParse(VideoGenFormSchema, data);
    if (!result.success) {
      throw new Error("Invalid image generation data");
    }
    return new VideoGenFormClass(
      result.output.name,
      result.output.prompt,
      result.output.negative_prompt,
      result.output.aspect_ratio,
      result.output.duration,
      result.output.enhance_prompt,
      result.output.generate_audio
    );
  }
}
