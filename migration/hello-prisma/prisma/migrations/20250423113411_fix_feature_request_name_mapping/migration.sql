/*
  Warnings:

  - The `status` column on the `FeatureRequest` table would be dropped and recreated. This will lead to data loss if there is data in the column.

*/
-- CreateEnum
CREATE TYPE "feature_status" AS ENUM ('Suggested', 'Planned', 'In_progress', 'Completed', 'Rejected');

-- AlterTable
ALTER TABLE "FeatureRequest" DROP COLUMN "status",
ADD COLUMN     "status" "feature_status" NOT NULL DEFAULT 'Suggested';

-- DropEnum
DROP TYPE "FeatureStatus";

-- CreateIndex
CREATE INDEX "FeatureRequest_status_idx" ON "FeatureRequest"("status");
