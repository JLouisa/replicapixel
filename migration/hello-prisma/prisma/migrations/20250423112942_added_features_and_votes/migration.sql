/*
  Warnings:

  - You are about to alter the column `message` on the `Notification` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `description` on the `Packs` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.

*/
-- CreateEnum
CREATE TYPE "FeatureStatus" AS ENUM ('Suggested', 'Planned', 'In_progress', 'Completed', 'Rejected');

-- AlterTable
ALTER TABLE "Notification" ALTER COLUMN "message" SET DATA TYPE VARCHAR(255);

-- AlterTable
ALTER TABLE "Packs" ALTER COLUMN "description" SET DATA TYPE VARCHAR(255);

-- CreateTable
CREATE TABLE "FeatureRequest" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "title" VARCHAR(255) NOT NULL,
    "description" TEXT NOT NULL,
    "status" "FeatureStatus" NOT NULL DEFAULT 'Suggested',
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "FeatureRequest_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "FeatureVote" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "featureRequest_id" INTEGER NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "FeatureVote_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "FeatureRequest_user_id_idx" ON "FeatureRequest"("user_id");

-- CreateIndex
CREATE INDEX "FeatureRequest_status_idx" ON "FeatureRequest"("status");

-- CreateIndex
CREATE INDEX "FeatureRequest_createdAt_idx" ON "FeatureRequest"("createdAt");

-- CreateIndex
CREATE INDEX "FeatureVote_user_id_idx" ON "FeatureVote"("user_id");

-- CreateIndex
CREATE INDEX "FeatureVote_featureRequest_id_idx" ON "FeatureVote"("featureRequest_id");

-- CreateIndex
CREATE UNIQUE INDEX "FeatureVote_user_id_featureRequest_id_key" ON "FeatureVote"("user_id", "featureRequest_id");

-- AddForeignKey
ALTER TABLE "FeatureRequest" ADD CONSTRAINT "FeatureRequest_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FeatureVote" ADD CONSTRAINT "FeatureVote_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FeatureVote" ADD CONSTRAINT "FeatureVote_featureRequest_id_fkey" FOREIGN KEY ("featureRequest_id") REFERENCES "FeatureRequest"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
