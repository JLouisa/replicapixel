import { object, string, union, type InferInput } from "valibot";
import { pipe, minLength, number, minValue, maxValue, length } from "valibot";
import { boolean, literal, safeParse } from "valibot";
// Define the Zod schema for the form data
const TrainingModelFormSchema = object({
  name: pipe(string(), minLength(2, "Name must be at least 2 characters.")),
  sex: pipe(string(), minLength(4, "Sex is required")),
  age: union([
    pipe(
      number(),
      minValue(18, "Age must be at least 18."),
      maxValue(100, "Age must not exceed 100.")
    ),
    string(),
  ]),
  based_on: string(),
  ethnicity: pipe(string(), length(8, "Ethnicity is required")),
  eye_color: pipe(string(), length(8, "Eye color is required")),
  creative: union([pipe(number(), minValue(100, "Creative must be a positive number")), string()]),
  bald: boolean(),
  consent: boolean(),
  file_type: literal("zip"),
});

// Define the TypeScript type for the form data
type TrainingModelFormData = InferInput<typeof TrainingModelFormSchema>;

// Define the TrainingModelForm class
export class TrainingModelFormClass implements TrainingModelFormData {
  static file_type: "zip" = "zip";

  constructor(
    public name: string,
    public sex: string,
    public age: number | string,
    public based_on: string,
    public ethnicity: string,
    public eye_color: string,
    public creative: number | string,
    public bald: boolean,
    public consent: boolean,
    public file_type: "zip" = this.file_type
  ) {}

  // Static method to create an instance after validation
  static create(data: unknown): TrainingModelFormClass {
    // Validate the input data using Valibot
    const validated = safeParse(TrainingModelFormSchema, data);

    const validatedData = validated.output as TrainingModelFormClass;
    return new TrainingModelFormClass(
      validatedData.name,
      validatedData.sex,
      Number(validatedData.age),
      validatedData.based_on,
      validatedData.ethnicity,
      validatedData.eye_color,
      Number(validatedData.creative),
      validatedData.bald,
      validatedData.consent,
      validatedData.file_type
    );
  }

  static inti(): TrainingModelFormClass {
    return new TrainingModelFormClass("", "", "", "", "", "", 40, false, false);
  }
}
