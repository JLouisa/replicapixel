/*
  Warnings:

  - You are about to alter the column `user_prompt` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `sys_prompt` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `content_type` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `image_size` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `fal_ai_request_id` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `image_url` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `image_url_s3` on the `Images` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `name` on the `Packs` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `description` on the `Packs` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `pack_prompts` on the `Packs` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `image_url` on the `Packs` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `name` on the `TrainingModels` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `thumbnail` on the `TrainingModels` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `fal_ai_request_id` on the `TrainingModels` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `s3_key` on the `TrainingModels` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `tensor_path` on the `TrainingModels` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `trigger_word` on the `TrainingModels` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `currency` on the `Transactions` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `payment_id` on the `Transactions` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `order_id` on the `Transactions` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `plan` on the `Transactions` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `name` on the `Users` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `email_verification_token` on the `Users` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `magicLink_token` on the `Users` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `reset_token` on the `Users` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - Changed the type of `age` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- AlterTable
ALTER TABLE "Images" ALTER COLUMN "user_prompt" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "sys_prompt" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "content_type" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "image_size" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "fal_ai_request_id" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "image_url" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "image_url_s3" SET DATA TYPE VARCHAR(255);

-- AlterTable
ALTER TABLE "Packs" ALTER COLUMN "name" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "description" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "pack_prompts" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "image_url" SET DATA TYPE VARCHAR(255);

-- AlterTable
ALTER TABLE "TrainingModels" ALTER COLUMN "name" SET DATA TYPE VARCHAR(255),
DROP COLUMN "age",
ADD COLUMN     "age" INTEGER NOT NULL,
ALTER COLUMN "thumbnail" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "fal_ai_request_id" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "s3_key" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "tensor_path" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "trigger_word" SET DATA TYPE VARCHAR(255);

-- AlterTable
ALTER TABLE "Transactions" ALTER COLUMN "currency" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "payment_id" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "order_id" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "plan" SET DATA TYPE VARCHAR(255);

-- AlterTable
ALTER TABLE "Users" ALTER COLUMN "name" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "email_verification_token" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "magicLink_token" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "reset_token" SET DATA TYPE VARCHAR(255);
