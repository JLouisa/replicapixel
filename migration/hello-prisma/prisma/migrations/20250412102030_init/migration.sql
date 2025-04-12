-- CreateEnum
CREATE TYPE "status" AS ENUM ('Pending', 'Processing', 'Training', 'Completed', 'Failed', 'Cancelled');

-- CreateEnum
CREATE TYPE "plan_names" AS ENUM ('Basic', 'Premium', 'Max');

-- CreateEnum
CREATE TYPE "sex" AS ENUM ('Male', 'Female');

-- CreateEnum
CREATE TYPE "ethnicity" AS ENUM ('White', 'Black', 'Pacific', 'Hispanic', 'Asian', 'SouthEastAsian', 'SouthAsian', 'MiddleEastern');

-- CreateEnum
CREATE TYPE "based_on" AS ENUM ('RealPerson', 'CreateInfluencerAI');

-- CreateEnum
CREATE TYPE "eye_color" AS ENUM ('Brown', 'Blue', 'Green', 'Grey', 'Hazel', 'Red');

-- CreateEnum
CREATE TYPE "emotion" AS ENUM ('Neutral', 'Anger', 'Disgust', 'Fear', 'Happy', 'Sad', 'Surprise');

-- CreateEnum
CREATE TYPE "image_size" AS ENUM ('Square', 'SquareHD', 'Portrait43', 'Portrait169', 'Landscape43', 'Landscape169');

-- CreateEnum
CREATE TYPE "image_format" AS ENUM ('png', 'jpg', 'zip');

-- CreateTable
CREATE TABLE "Users" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "api_key" VARCHAR(255) NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "stripe_customer_id" VARCHAR(255),
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
    "sex" "sex" NOT NULL,
    "ethnicity" "ethnicity" NOT NULL,
    "basedOn" "based_on" NOT NULL,
    "eye_color" "eye_color" NOT NULL,
    "bald" BOOLEAN NOT NULL DEFAULT false,
    "steps" INTEGER NOT NULL,
    "create_mask" BOOLEAN NOT NULL,
    "is_style" BOOLEAN NOT NULL,
    "trigger_word" VARCHAR(255) NOT NULL,
    "tensor_path" TEXT,
    "thumbnail" TEXT,
    "training_status" "status" NOT NULL,
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
    "alt" TEXT NOT NULL,
    "num_inference_steps" INTEGER NOT NULL,
    "content_type" "image_format" NOT NULL,
    "status" "status" NOT NULL,
    "image_size" "image_size" NOT NULL,
    "fal_ai_request_id" VARCHAR(255),
    "width" INTEGER,
    "height" INTEGER,
    "image_s3_key" VARCHAR(255) NOT NULL,
    "image_url_fal" TEXT,
    "is_favorite" BOOLEAN NOT NULL DEFAULT true,
    "deleted_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Images_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Plans" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "plan_name" "plan_names" NOT NULL,
    "credit_amount" INTEGER NOT NULL,
    "model_amount" INTEGER NOT NULL,
    "price_cents" INTEGER NOT NULL,
    "stripe_price_id" TEXT NOT NULL,
    "subtitle" VARCHAR(255) NOT NULL,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "cta" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_popular" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Plans_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Transactions" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "plan_id" INTEGER NOT NULL,
    "credit_amount" INTEGER NOT NULL,
    "model_amount" INTEGER NOT NULL,
    "currency" VARCHAR(16) NOT NULL,
    "payment_id" VARCHAR(255) NOT NULL,
    "status" "status" NOT NULL,
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

-- CreateTable
CREATE TABLE "handled_stripe_events" (
    "session_id" TEXT NOT NULL,
    "processed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "handled_stripe_events_pkey" PRIMARY KEY ("session_id")
);

-- CreateTable
CREATE TABLE "handled_fal_events" (
    "request_id" TEXT NOT NULL,
    "processed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "handled_fal_events_pkey" PRIMARY KEY ("request_id")
);

-- CreateTable
CREATE TABLE "seaql_migrations" (
    "version" VARCHAR NOT NULL,
    "applied_at" BIGINT NOT NULL,

    CONSTRAINT "seaql_migrations_pkey" PRIMARY KEY ("version")
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
CREATE UNIQUE INDEX "Plans_pid_key" ON "Plans"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_name_key" ON "Plans"("name");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_plan_name_key" ON "Plans"("plan_name");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_stripe_price_id_key" ON "Plans"("stripe_price_id");

-- CreateIndex
CREATE INDEX "idx-plan-pid" ON "Plans"("pid");

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
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_plan_id_fkey" FOREIGN KEY ("plan_id") REFERENCES "Plans"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
