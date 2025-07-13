-- CreateEnum
CREATE TYPE "aspect_ratio" AS ENUM ('Widescreen', 'Portrait', 'Square');

-- AlterTable
ALTER TABLE "Images" ALTER COLUMN "is_favorite" SET DEFAULT false;

-- CreateTable
CREATE TABLE "Videos" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "user_prompt" TEXT NOT NULL,
    "sys_prompt" TEXT NOT NULL,
    "negative_prompt" TEXT,
    "alt" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 8,
    "enhance_prompt" BOOLEAN NOT NULL DEFAULT false,
    "generate_audio" BOOLEAN NOT NULL DEFAULT true,
    "seed" INTEGER,
    "status" "status" NOT NULL,
    "aspect_ratio" "aspect_ratio" NOT NULL,
    "video_cost" INTEGER NOT NULL DEFAULT 80,
    "fal_ai_request_id" VARCHAR(255),
    "image_s3_key" VARCHAR(255) NOT NULL,
    "image_url_fal" TEXT,
    "is_favorite" BOOLEAN NOT NULL DEFAULT false,
    "deleted_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Videos_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Videos_pid_key" ON "Videos"("pid");

-- CreateIndex
CREATE INDEX "idx-video-pid" ON "Videos"("pid");

-- CreateIndex
CREATE INDEX "idx-video-user_id" ON "Videos"("user_id");

-- CreateIndex
CREATE INDEX "idx-fal_ai_request_id-video" ON "Videos"("fal_ai_request_id");

-- AddForeignKey
ALTER TABLE "Videos" ADD CONSTRAINT "Videos_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
