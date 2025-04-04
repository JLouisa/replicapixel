-- CreateEnum
CREATE TYPE "Sex" AS ENUM ('Male', 'Female');

-- CreateEnum
CREATE TYPE "Ethnicity" AS ENUM ('White', 'Black', 'Pacific', 'Hispanic', 'Asian', 'SouthEastAsian', 'SouthAsian', 'MiddleEastern');

-- CreateEnum
CREATE TYPE "BasedOn" AS ENUM ('RealPerson', 'CreateInfluencerAI');

-- CreateEnum
CREATE TYPE "EyeColor" AS ENUM ('Brown', 'Blue', 'Green', 'Grey', 'Hazel', 'Red');

-- CreateEnum
CREATE TYPE "Emotion" AS ENUM ('Neutral', 'Anger', 'Disgust', 'Fear', 'Happy', 'Sad', 'Surprise');

-- CreateEnum
CREATE TYPE "Status" AS ENUM ('Pending', 'Processing', 'Training', 'Completed', 'Failed', 'Cancelled');

-- CreateEnum
CREATE TYPE "ImageSize" AS ENUM ('Square', 'SquareHD', 'Portrait43', 'Portrait169', 'Landscape43', 'Landscape169');

-- CreateEnum
CREATE TYPE "ImageFormat" AS ENUM ('Png', 'Jpeg', 'Zip');

-- CreateTable
CREATE TABLE "Users" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "api_key" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "reset_token" VARCHAR(255),
    "reset_sent_at" TIMESTAMPTZ,
    "email_verification_token" VARCHAR(255),
    "email_verification_sent_at" TIMESTAMPTZ,
    "email_verified_at" TIMESTAMPTZ,
    "magicLink_token" VARCHAR(255),
    "magicLink_expiration" TIMESTAMPTZ,

    CONSTRAINT "Users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "TrainingModels" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "age" INTEGER NOT NULL,
    "sex" "Sex" NOT NULL,
    "ethnicity" "Ethnicity" NOT NULL,
    "basedOn" "BasedOn" NOT NULL,
    "eye_color" "EyeColor" NOT NULL,
    "bald" BOOLEAN NOT NULL DEFAULT false,
    "steps" INTEGER NOT NULL,
    "create_mask" BOOLEAN NOT NULL,
    "is_style" BOOLEAN NOT NULL,
    "trigger_word" VARCHAR(255) NOT NULL,
    "tensor_path" TEXT,
    "thumbnail" TEXT,
    "training_status" "Status" NOT NULL,
    "fal_output" JSONB,
    "training_images" JSONB,
    "fal_ai_request_id" VARCHAR(255),
    "s3_key" VARCHAR(255) NOT NULL,
    "is_verified" BOOLEAN NOT NULL DEFAULT false,
    "deleted_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "TrainingModels_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserCredits" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "credit_amount" INTEGER NOT NULL DEFAULT 0,
    "model_amount" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "UserCredits_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Images" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "training_model_id" INTEGER NOT NULL,
    "pack_id" INTEGER,
    "user_prompt" TEXT NOT NULL,
    "sys_prompt" TEXT NOT NULL,
    "num_inference_steps" INTEGER NOT NULL,
    "content_type" "ImageFormat" NOT NULL,
    "status" "Status" NOT NULL,
    "image_size" "ImageSize" NOT NULL,
    "fal_ai_request_id" VARCHAR(255),
    "width" INTEGER,
    "height" INTEGER,
    "image_url" TEXT,
    "image_url_s3" TEXT,
    "is_favorite" BOOLEAN NOT NULL DEFAULT true,
    "deleted_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Images_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Transactions" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "credit_amount" INTEGER NOT NULL,
    "model_amount" INTEGER NOT NULL,
    "currency" VARCHAR(255) NOT NULL,
    "payment_id" VARCHAR(255) NOT NULL,
    "order_id" VARCHAR(255) NOT NULL,
    "plan" VARCHAR(255) NOT NULL,
    "status" "Status" NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Transactions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Packs" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT NOT NULL,
    "pack_prompts" TEXT NOT NULL,
    "credits" INTEGER NOT NULL,
    "amount" INTEGER NOT NULL,
    "image_url" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Packs_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Users_pid_key" ON "Users"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Users_email_key" ON "Users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "Users_api_key_key" ON "Users"("api_key");

-- CreateIndex
CREATE INDEX "idx-users-pid" ON "Users"("pid");

-- CreateIndex
CREATE INDEX "idx-users-email" ON "Users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "TrainingModels_pid_key" ON "TrainingModels"("pid");

-- CreateIndex
CREATE INDEX "idx-training_models-pid" ON "TrainingModels"("pid");

-- CreateIndex
CREATE INDEX "idx-user_id-id" ON "TrainingModels"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "UserCredits_pid_key" ON "UserCredits"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "UserCredits_user_id_key" ON "UserCredits"("user_id");

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
ALTER TABLE "TrainingModels" ADD CONSTRAINT "TrainingModels_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserCredits" ADD CONSTRAINT "UserCredits_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_training_model_id_fkey" FOREIGN KEY ("training_model_id") REFERENCES "TrainingModels"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_pack_id_fkey" FOREIGN KEY ("pack_id") REFERENCES "Packs"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
