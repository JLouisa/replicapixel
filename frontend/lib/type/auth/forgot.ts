import { safeParse, type InferInput } from "valibot";
import { ForgotFormSchema } from "./baseAuth";

export type ForgotForm = InferInput<typeof ForgotFormSchema>;

export class ForgotFormClass implements ForgotForm {
  email: string;

  constructor(email: string) {
    this.email = email;
  }

  private static vali(data: any): ForgotForm {
    const vali = safeParse(ForgotFormSchema, data);
    if (vali.success) {
      return vali.output;
    } else {
      const errorDetails = vali.issues;
      throw new Error(`ForgotForm Parsing Failed: ${errorDetails}`);
    }
  }

  static newToServer(form: unknown): ForgotFormClass {
    try {
      const { email } = ForgotFormClass.vali(form);
      return new ForgotFormClass(email);
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  static create(email: string): ForgotForm {
    return new ForgotFormClass(email);
  }
}
