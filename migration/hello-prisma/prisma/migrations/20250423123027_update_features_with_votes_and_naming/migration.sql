/*
  Warnings:

  - You are about to drop the column `createdAt` on the `FeatureRequest` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `FeatureRequest` table. All the data in the column will be lost.
  - You are about to drop the column `createdAt` on the `FeatureVote` table. All the data in the column will be lost.
  - You are about to drop the column `featureRequest_id` on the `FeatureVote` table. All the data in the column will be lost.
  - You are about to drop the column `updatedAt` on the `FeatureVote` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[user_id,feature_request_id]` on the table `FeatureVote` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `votes` to the `FeatureRequest` table without a default value. This is not possible if the table is not empty.
  - Added the required column `feature_request_id` to the `FeatureVote` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "FeatureVote" DROP CONSTRAINT "FeatureVote_featureRequest_id_fkey";

-- DropIndex
DROP INDEX "FeatureRequest_createdAt_idx";

-- DropIndex
DROP INDEX "FeatureVote_featureRequest_id_idx";

-- DropIndex
DROP INDEX "FeatureVote_user_id_featureRequest_id_key";

-- AlterTable
ALTER TABLE "FeatureRequest" DROP COLUMN "createdAt",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "votes" INTEGER NOT NULL;

-- AlterTable
ALTER TABLE "FeatureVote" DROP COLUMN "createdAt",
DROP COLUMN "featureRequest_id",
DROP COLUMN "updatedAt",
ADD COLUMN     "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "feature_request_id" INTEGER NOT NULL,
ADD COLUMN     "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- CreateIndex
CREATE INDEX "FeatureRequest_created_at_idx" ON "FeatureRequest"("created_at");

-- CreateIndex
CREATE INDEX "FeatureVote_feature_request_id_idx" ON "FeatureVote"("feature_request_id");

-- CreateIndex
CREATE UNIQUE INDEX "FeatureVote_user_id_feature_request_id_key" ON "FeatureVote"("user_id", "feature_request_id");

-- AddForeignKey
ALTER TABLE "FeatureVote" ADD CONSTRAINT "FeatureVote_feature_request_id_fkey" FOREIGN KEY ("feature_request_id") REFERENCES "FeatureRequest"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
