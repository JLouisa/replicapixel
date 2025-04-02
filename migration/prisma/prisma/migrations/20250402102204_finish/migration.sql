/*
  Warnings:

  - Changed the type of `training_status` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "Status" AS ENUM ('Pending', 'Processing', 'Training', 'Completed', 'Failed', 'Cancelled');

-- AlterTable
ALTER TABLE "TrainingModels" DROP COLUMN "training_status",
ADD COLUMN     "training_status" "Status" NOT NULL;

-- CreateTable
CREATE TABLE "Images" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "training_model_id" INTEGER NOT NULL,
    "user_prompt" TEXT NOT NULL,
    "sys_prompt" TEXT NOT NULL,
    "num_inference_steps" INTEGER NOT NULL,
    "content_type" TEXT NOT NULL,
    "status" "Status" NOT NULL,
    "image_size" TEXT NOT NULL,
    "fal_ai_request_id" TEXT NOT NULL,
    "width" INTEGER NOT NULL,
    "height" INTEGER NOT NULL,
    "image_url" TEXT NOT NULL,
    "image_url_s3" TEXT NOT NULL,
    "is_favorite" BOOLEAN NOT NULL,
    "deleted_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "Images_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Transactions" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "credit_amount" INTEGER NOT NULL,
    "model_amount" INTEGER NOT NULL,
    "currency" TEXT NOT NULL,
    "payment_id" TEXT NOT NULL,
    "order_id" TEXT NOT NULL,
    "plan" TEXT NOT NULL,
    "status" "Status" NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "Transactions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Packs" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "pack_prompts" TEXT NOT NULL,
    "credits" INTEGER NOT NULL,
    "amount" INTEGER NOT NULL,
    "image_url" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "Packs_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Images_pid_key" ON "Images"("pid");

-- CreateIndex
CREATE INDEX "idx-images-pid" ON "Images"("pid");

-- CreateIndex
CREATE INDEX "idx-images-user_id" ON "Images"("user_id");

-- CreateIndex
CREATE INDEX "idx-images-training_model_id" ON "Images"("training_model_id");

-- CreateIndex
CREATE UNIQUE INDEX "Transactions_pid_key" ON "Transactions"("pid");

-- CreateIndex
CREATE INDEX "idx-transactions-pid" ON "Transactions"("pid");

-- CreateIndex
CREATE INDEX "idx-transactions-user_id" ON "Transactions"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "Packs_pid_key" ON "Packs"("pid");

-- CreateIndex
CREATE INDEX "idx-packs-pid" ON "Packs"("pid");

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_training_model_id_fkey" FOREIGN KEY ("training_model_id") REFERENCES "TrainingModels"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
