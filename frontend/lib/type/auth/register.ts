import { safeParse, type InferInput } from "valibot";
import { RegisterFormSchema } from "./baseAuth";

export type RegisterForm = InferInput<typeof RegisterFormSchema>;

export class RegisterFormClass implements RegisterForm {
  name: string;
  email: string;
  password: string;
  confirmPassword: string;

  constructor(name: string, email: string, password: string, confirmPassword: string) {
    this.name = name;
    this.email = email;
    this.password = password;
    this.confirmPassword = confirmPassword;
  }

  private static vali(data: any): RegisterForm {
    const vali = safeParse(RegisterFormSchema, data);
    if (vali.success) {
      return vali.output;
    } else {
      const errorDetails = vali.issues;
      throw new Error(`RegisterForm Parsing Failed: ${errorDetails}`);
    }
  }

  static newToServer(form: unknown): RegisterFormClass {
    try {
      const { name, email, password, confirmPassword } = RegisterFormClass.vali(form);
      return new RegisterFormClass(name, email, password, confirmPassword);
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  static create(
    name: string,
    email: string,
    password: string,
    confirmPassword: string
  ): RegisterForm {
    return new RegisterFormClass(name, email, password, confirmPassword);
  }
}
