import { safeParse, type InferInput } from "valibot";
import { LoginFormSchema } from "./baseAuth";

export type LoginForm = InferInput<typeof LoginFormSchema>;

export class LoginFormClass implements LoginForm {
  email: string;
  password: string;
  remember: boolean;

  constructor(email: string, password: string, remember: boolean) {
    this.email = email;
    this.password = password;
    this.remember = remember;
  }

  private static vali(data: any): LoginForm {
    const vali = safeParse(LoginFormSchema, data);
    if (vali.success) {
      return vali.output;
    } else {
      const errorDetails = vali.issues;
      throw new Error(`LoginForm Parsing Failed: ${errorDetails}`);
    }
  }

  static newToServer(form: unknown): LoginFormClass {
    try {
      const { email, password, remember } = LoginFormClass.vali(form);
      return new LoginFormClass(email, password, remember);
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  static create(email: string, password: string, remember: boolean): LoginForm {
    return new LoginFormClass(email, password, remember);
  }
}
