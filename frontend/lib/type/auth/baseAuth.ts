import { object, string, boolean, email, minLength } from "valibot";
import { maxLength, pipe, pick, omit, check } from "valibot";

// Base form schema
const baseFormSchema = object({
  name: pipe(
    string(),
    minLength(2, "Name must be at least 2 characters long"),
    maxLength(50, "Name cannot exceed 50 characters")
  ),
  email: pipe(
    string(),
    email("Please enter a valid email address"),
    minLength(9, "Email must be at least 9 characters long"),
    maxLength(50, "Email cannot exceed 50 characters")
  ),
  password: pipe(
    string(),
    minLength(9, "Password must be at least 9 characters long"),
    maxLength(50, "Password cannot exceed 50 characters")
  ),
  confirmPassword: pipe(
    string(),
    minLength(9, "Password must be at least 9 characters long"),
    maxLength(50, "Password cannot exceed 50 characters")
  ),
  remember: boolean(),
});

// Sign-in form schema (email, password, remember)
export const RegisterFormSchema = pipe(
  omit(baseFormSchema, ["remember"]),
  check((input) => input.password === input.confirmPassword, "Password does not match.")
);
export const LoginFormSchema = pick(baseFormSchema, ["email", "password", "remember"]);
export const ForgotFormSchema = pick(baseFormSchema, ["email"]);
