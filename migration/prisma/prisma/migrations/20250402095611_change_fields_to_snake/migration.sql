/*
  Warnings:

  - You are about to drop the column `createMask` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `deletedAt` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `eyeColor` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `falAiRequestId` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `falOutput` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `isStyle` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `isVerified` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `s3Key` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `tensorPath` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `trainingImages` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `trainingStatus` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `triggerWord` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `userId` on the `TrainingModels` table. All the data in the column will be lost.
  - You are about to drop the column `apiKey` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `emailVerificationSentAt` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `emailVerificationToken` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `emailVerifiedAt` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `magicLinkExpiration` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `magicLinkToken` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `resetSentAt` on the `Users` table. All the data in the column will be lost.
  - You are about to drop the column `resetToken` on the `Users` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[api_key]` on the table `Users` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `create_mask` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `eye_color` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `is_style` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `s3_key` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `training_status` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `trigger_word` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `updated_at` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `user_id` to the `TrainingModels` table without a default value. This is not possible if the table is not empty.
  - Added the required column `api_key` to the `Users` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "TrainingModels" DROP CONSTRAINT "TrainingModels_userId_fkey";

-- DropIndex
DROP INDEX "TrainingModels_userId_key";

-- DropIndex
DROP INDEX "Users_apiKey_key";

-- AlterTable
ALTER TABLE "TrainingModels" DROP COLUMN "createMask",
DROP COLUMN "createdAt",
DROP COLUMN "deletedAt",
DROP COLUMN "eyeColor",
DROP COLUMN "falAiRequestId",
DROP COLUMN "falOutput",
DROP COLUMN "isStyle",
DROP COLUMN "isVerified",
DROP COLUMN "s3Key",
DROP COLUMN "tensorPath",
DROP COLUMN "trainingImages",
DROP COLUMN "trainingStatus",
DROP COLUMN "triggerWord",
DROP COLUMN "updatedAt",
DROP COLUMN "userId",
ADD COLUMN     "create_mask" BOOLEAN NOT NULL,
ADD COLUMN     "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "deleted_at" TIMESTAMPTZ,
ADD COLUMN     "eye_color" "EyeColor" NOT NULL,
ADD COLUMN     "fal_ai_request_id" TEXT,
ADD COLUMN     "fal_output" JSONB,
ADD COLUMN     "is_style" BOOLEAN NOT NULL,
ADD COLUMN     "is_verified" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "s3_key" TEXT NOT NULL,
ADD COLUMN     "tensor_path" TEXT,
ADD COLUMN     "training_images" JSONB,
ADD COLUMN     "training_status" TEXT NOT NULL,
ADD COLUMN     "trigger_word" TEXT NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMPTZ NOT NULL,
ADD COLUMN     "user_id" INTEGER NOT NULL;

-- AlterTable
ALTER TABLE "Users" DROP COLUMN "apiKey",
DROP COLUMN "emailVerificationSentAt",
DROP COLUMN "emailVerificationToken",
DROP COLUMN "emailVerifiedAt",
DROP COLUMN "magicLinkExpiration",
DROP COLUMN "magicLinkToken",
DROP COLUMN "resetSentAt",
DROP COLUMN "resetToken",
ADD COLUMN     "api_key" TEXT NOT NULL,
ADD COLUMN     "email_verification_sent_at" TIMESTAMPTZ,
ADD COLUMN     "email_verification_token" TEXT,
ADD COLUMN     "email_verified_at" TIMESTAMPTZ,
ADD COLUMN     "magicLink_expiration" TIMESTAMPTZ,
ADD COLUMN     "magicLink_token" TEXT,
ADD COLUMN     "reset_sent_at" TIMESTAMPTZ,
ADD COLUMN     "reset_token" TEXT;

-- CreateTable
CREATE TABLE "UserCredits" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "credit_amount" INTEGER NOT NULL DEFAULT 0,
    "model_amount" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "UserCredits_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "UserCredits_pid_key" ON "UserCredits"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "UserCredits_user_id_key" ON "UserCredits"("user_id");

-- CreateIndex
CREATE INDEX "idx-user_id-id" ON "TrainingModels"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "Users_api_key_key" ON "Users"("api_key");

-- AddForeignKey
ALTER TABLE "TrainingModels" ADD CONSTRAINT "TrainingModels_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserCredits" ADD CONSTRAINT "UserCredits_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
