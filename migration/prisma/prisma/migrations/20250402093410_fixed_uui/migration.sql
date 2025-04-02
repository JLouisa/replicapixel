/*
  Warnings:

  - Changed the type of `pid` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `userId` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Made the column `eyeColor` on table `TrainingModels` required. This step will fail if there are existing NULL values in that column.
  - Changed the type of `pid` on the `Users` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- DropForeignKey
ALTER TABLE "TrainingModels" DROP CONSTRAINT "TrainingModels_userId_fkey";

-- AlterTable
ALTER TABLE "TrainingModels" ADD COLUMN     "deletedAt" TIMESTAMPTZ,
DROP COLUMN "pid",
ADD COLUMN     "pid" UUID NOT NULL,
DROP COLUMN "userId",
ADD COLUMN     "userId" UUID NOT NULL,
ALTER COLUMN "eyeColor" SET NOT NULL,
ALTER COLUMN "isVerified" SET DEFAULT false;

-- AlterTable
ALTER TABLE "Users" DROP COLUMN "pid",
ADD COLUMN     "pid" UUID NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "TrainingModels_pid_key" ON "TrainingModels"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "TrainingModels_userId_key" ON "TrainingModels"("userId");

-- CreateIndex
CREATE INDEX "idx-training_models-pid" ON "TrainingModels"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Users_pid_key" ON "Users"("pid");

-- CreateIndex
CREATE INDEX "idx-users-pid" ON "Users"("pid");

-- AddForeignKey
ALTER TABLE "TrainingModels" ADD CONSTRAINT "TrainingModels_userId_fkey" FOREIGN KEY ("userId") REFERENCES "Users"("pid") ON DELETE CASCADE ON UPDATE CASCADE;
