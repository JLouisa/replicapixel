-- CreateEnum
CREATE TYPE "account" AS ENUM ('Website', 'Google', 'Github');

-- CreateEnum
CREATE TYPE "based_on" AS ENUM ('RealPerson', 'CreateInfluencerAI');

-- CreateEnum
CREATE TYPE "emotion" AS ENUM ('Neutral', 'Anger', 'Disgust', 'Fear', 'Happy', 'Sad', 'Surprise');

-- CreateEnum
CREATE TYPE "ethnicity" AS ENUM ('White', 'Black', 'Pacific', 'Hispanic', 'Asian', 'SouthEastAsian', 'SouthAsian', 'MiddleEastern');

-- CreateEnum
CREATE TYPE "eye_color" AS ENUM ('Brown', 'Blue', 'Green', 'Grey', 'Hazel', 'Red');

-- CreateEnum
CREATE TYPE "feature_status" AS ENUM ('Suggested', 'Planned', 'In_progress', 'Completed', 'Rejected');

-- CreateEnum
CREATE TYPE "image_format" AS ENUM ('png', 'jpg', 'zip');

-- CreateEnum
CREATE TYPE "image_size" AS ENUM ('Square', 'SquareHD', 'Portrait43', 'Portrait169', 'Landscape43', 'Landscape169');

-- CreateEnum
CREATE TYPE "language" AS ENUM ('English', 'Spanish', 'German', 'Italian', 'Dutch');

-- CreateEnum
CREATE TYPE "notification" AS ENUM ('Message', 'System_update', 'Promotion');

-- CreateEnum
CREATE TYPE "plan_names" AS ENUM ('Basic', 'Premium', 'Max');

-- CreateEnum
CREATE TYPE "sex" AS ENUM ('Male', 'Female');

-- CreateEnum
CREATE TYPE "status" AS ENUM ('Pending', 'Processing', 'Training', 'Completed', 'Failed', 'Cancelled');

-- CreateEnum
CREATE TYPE "theme_preference" AS ENUM ('Light', 'Dark', 'System');

-- CreateTable
CREATE TABLE "FeatureRequest" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "title" VARCHAR(255) NOT NULL,
    "description" TEXT NOT NULL,
    "status" "feature_status" NOT NULL DEFAULT 'Suggested',
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "votes" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "FeatureRequest_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "FeatureVote" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "feature_request_id" INTEGER NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "FeatureVote_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Images" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "training_model_id" INTEGER,
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
    "deleted_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "image_cost" INTEGER NOT NULL DEFAULT 2,

    CONSTRAINT "Images_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Notification" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "message" VARCHAR(255) NOT NULL,
    "read" BOOLEAN NOT NULL DEFAULT false,
    "link" TEXT,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "type" "notification" NOT NULL DEFAULT 'Message',

    CONSTRAINT "Notification_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Packs" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "pack_prompts" TEXT NOT NULL,
    "credits" INTEGER NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "title" VARCHAR(255) NOT NULL,
    "full_description" TEXT NOT NULL,
    "short_description" VARCHAR(255) NOT NULL,
    "num_images" INTEGER NOT NULL,
    "num_inference_steps" INTEGER NOT NULL DEFAULT 50,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "images" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "main_image" TEXT NOT NULL DEFAULT 'url',
    "used" INTEGER NOT NULL DEFAULT 0,
    "stars" INTEGER NOT NULL DEFAULT 5,
    "popular" BOOLEAN NOT NULL DEFAULT false,
    "title_url" VARCHAR(255) NOT NULL,

    CONSTRAINT "Packs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Packs_translations" (
    "id" SERIAL NOT NULL,
    "pack_id" INTEGER NOT NULL,
    "language" "language" NOT NULL,
    "title" VARCHAR(255) NOT NULL,
    "short_description" VARCHAR(255) NOT NULL,
    "full_description" TEXT NOT NULL,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Packs_translations_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Plans" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "plan_name" "plan_names" NOT NULL,
    "credit_amount" INTEGER NOT NULL,
    "model_amount" INTEGER NOT NULL,
    "price_cents" BIGINT NOT NULL,
    "stripe_price_id" TEXT NOT NULL,
    "subtitle" VARCHAR(255) NOT NULL,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "cta" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_popular" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Plans_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Plans_translations" (
    "id" SERIAL NOT NULL,
    "plan_id" INTEGER NOT NULL,
    "language" "language" NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "subtitle" VARCHAR(255) NOT NULL,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "cta" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Plans_translations_pkey" PRIMARY KEY ("id")
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
    "deleted_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "TrainingModels_pkey" PRIMARY KEY ("id")
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
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "payment_amount" BIGINT NOT NULL,

    CONSTRAINT "Transactions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserCredits" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "credit_amount" INTEGER NOT NULL DEFAULT 0,
    "model_amount" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "UserCredits_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserSettings" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "enable_notification_email" BOOLEAN NOT NULL DEFAULT true,
    "enable_marketing_email" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "language" "language" NOT NULL DEFAULT 'English',
    "theme" "theme_preference" NOT NULL DEFAULT 'System',

    CONSTRAINT "UserSettings_pkey" PRIMARY KEY ("id")
);

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
    "reset_sent_at" TIMESTAMPTZ(6),
    "email_verification_token" VARCHAR(255),
    "email_verification_sent_at" TIMESTAMPTZ(6),
    "email_verified_at" TIMESTAMPTZ(6),
    "magicLink_token" VARCHAR(255),
    "magicLink_expiration" TIMESTAMPTZ(6),
    "picture" VARCHAR(255),
    "account" "account" NOT NULL DEFAULT 'Website',

    CONSTRAINT "Users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "handled_fal_events" (
    "request_id" TEXT NOT NULL,
    "processed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "handled_fal_events_pkey" PRIMARY KEY ("request_id")
);

-- CreateTable
CREATE TABLE "handled_stripe_events" (
    "session_id" TEXT NOT NULL,
    "processed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "handled_stripe_events_pkey" PRIMARY KEY ("session_id")
);

-- CreateTable
CREATE TABLE "o_auth2_sessions" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "session_id" VARCHAR(255) NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "o_auth2_sessions_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "FeatureRequest_created_at_idx" ON "FeatureRequest"("created_at");

-- CreateIndex
CREATE INDEX "FeatureRequest_status_idx" ON "FeatureRequest"("status");

-- CreateIndex
CREATE INDEX "FeatureRequest_user_id_idx" ON "FeatureRequest"("user_id");

-- CreateIndex
CREATE INDEX "FeatureVote_feature_request_id_idx" ON "FeatureVote"("feature_request_id");

-- CreateIndex
CREATE INDEX "FeatureVote_user_id_idx" ON "FeatureVote"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "FeatureVote_user_id_feature_request_id_key" ON "FeatureVote"("user_id", "feature_request_id");

-- CreateIndex
CREATE UNIQUE INDEX "Images_pid_key" ON "Images"("pid");

-- CreateIndex
CREATE INDEX "idx-fal_ai_request_id-pid" ON "Images"("fal_ai_request_id");

-- CreateIndex
CREATE INDEX "idx-images-created_at" ON "Images"("created_at");

-- CreateIndex
CREATE INDEX "idx-images-pid" ON "Images"("pid");

-- CreateIndex
CREATE INDEX "idx-images-training_model_id" ON "Images"("training_model_id");

-- CreateIndex
CREATE INDEX "idx-images-updated_at" ON "Images"("updated_at");

-- CreateIndex
CREATE INDEX "idx-images-user_id" ON "Images"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "Notification_pid_key" ON "Notification"("pid");

-- CreateIndex
CREATE INDEX "Notification_user_id_idx" ON "Notification"("user_id");

-- CreateIndex
CREATE INDEX "Notification_user_id_read_idx" ON "Notification"("user_id", "read");

-- CreateIndex
CREATE UNIQUE INDEX "Packs_pid_key" ON "Packs"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Packs_title_url_key" ON "Packs"("title_url");

-- CreateIndex
CREATE INDEX "idx-packs-pid" ON "Packs"("pid");

-- CreateIndex
CREATE INDEX "idx-title-pid" ON "Packs"("title_url");

-- CreateIndex
CREATE INDEX "Packs_translations_pack_id_idx" ON "Packs_translations"("pack_id");

-- CreateIndex
CREATE UNIQUE INDEX "Packs_translations_pack_id_language_key" ON "Packs_translations"("pack_id", "language");

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
CREATE INDEX "Plans_translations_plan_id_idx" ON "Plans_translations"("plan_id");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_translations_plan_id_language_key" ON "Plans_translations"("plan_id", "language");

-- CreateIndex
CREATE UNIQUE INDEX "TrainingModels_pid_key" ON "TrainingModels"("pid");

-- CreateIndex
CREATE INDEX "idx-training_models-pid" ON "TrainingModels"("pid");

-- CreateIndex
CREATE INDEX "idx-user_id-id" ON "TrainingModels"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "Transactions_pid_key" ON "Transactions"("pid");

-- CreateIndex
CREATE INDEX "idx-transactions-pid" ON "Transactions"("pid");

-- CreateIndex
CREATE INDEX "idx-transactions-user_id" ON "Transactions"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "UserCredits_pid_key" ON "UserCredits"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "UserCredits_user_id_key" ON "UserCredits"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "UserSettings_user_id_key" ON "UserSettings"("user_id");

-- CreateIndex
CREATE INDEX "UserSettings_user_id_idx" ON "UserSettings"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "Users_pid_key" ON "Users"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Users_email_key" ON "Users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "Users_api_key_key" ON "Users"("api_key");

-- CreateIndex
CREATE INDEX "idx-users-email" ON "Users"("email");

-- CreateIndex
CREATE INDEX "idx-users-pid" ON "Users"("pid");

-- CreateIndex
CREATE INDEX "idx-oauth2-session_id" ON "o_auth2_sessions"("session_id");

-- AddForeignKey
ALTER TABLE "FeatureRequest" ADD CONSTRAINT "FeatureRequest_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FeatureVote" ADD CONSTRAINT "FeatureVote_feature_request_id_fkey" FOREIGN KEY ("feature_request_id") REFERENCES "FeatureRequest"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FeatureVote" ADD CONSTRAINT "FeatureVote_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_pack_id_fkey" FOREIGN KEY ("pack_id") REFERENCES "Packs"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_training_model_id_fkey" FOREIGN KEY ("training_model_id") REFERENCES "TrainingModels"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Images" ADD CONSTRAINT "Images_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Notification" ADD CONSTRAINT "Notification_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Packs_translations" ADD CONSTRAINT "Packs_translations_pack_id_fkey" FOREIGN KEY ("pack_id") REFERENCES "Packs"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Plans_translations" ADD CONSTRAINT "Plans_translations_plan_id_fkey" FOREIGN KEY ("plan_id") REFERENCES "Plans"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "TrainingModels" ADD CONSTRAINT "TrainingModels_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_plan_id_fkey" FOREIGN KEY ("plan_id") REFERENCES "Plans"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Transactions" ADD CONSTRAINT "Transactions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserCredits" ADD CONSTRAINT "UserCredits_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserSettings" ADD CONSTRAINT "UserSettings_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "o_auth2_sessions" ADD CONSTRAINT "o_auth2_sessions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

