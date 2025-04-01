import { array, enum_, nullable, object } from "valibot";
import { string, union, null as null_, type InferInput, parse } from "valibot";
import { PreSignUrlRequestSchema } from "./presignedUrlRequest_type";

export enum ResponseStatus {
  Success = "Success",
  Error = "Error",
}

export const ServerResponseSchema = object({
  status: enum_(ResponseStatus),
  message: nullable(string()),
  data: union([
    null_(), // Allow no data
    string(), // TODO: Remove and fix
    PreSignUrlRequestSchema,
    array(PreSignUrlRequestSchema),
  ]),
});
export type ServerResponse = InferInput<typeof ServerResponseSchema>;

export class ServerResponseClass<T> {
  private response: ServerResponse;

  constructor(response: any) {
    // Validate the response using Zod's `parse`, which throws an error if validation fails
    this.response = parse(ServerResponseSchema, response);

    // Ensure that data is not null if status is "Success"
    //TODO: ^^^ This is a workaround until we can fix the Zod schema for ServerResponse
    if (this.response.status === ResponseStatus.Success && this.response.data === null) {
      throw new Error("Internal Server Error! Please try again later");
    }

    // Ensure that data is not null if status is "Error" and throw the appropriate message
    if (this.response.status === ResponseStatus.Error) {
      throw new Error(`${this.response.message || "An error occurred"}`);
    }
  }

  // Method to check if the response was successful
  isSuccess(): boolean {
    return this.response.status === ResponseStatus.Success;
  }

  // Method to get the message from the response
  getMessage(): string | null {
    return this.response.message;
  }

  // Method to get the data from the response, guaranteed non-null when successful
  getData(): T {
    return this.response.data as T; // No need for an additional null check
  }
}
