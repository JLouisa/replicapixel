/*
  Warnings:

  - Changed the type of `status` on the `Images` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `training_status` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `status` on the `Transactions` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "status" AS ENUM ('Pending', 'Processing', 'Training', 'Completed', 'Failed', 'Cancelled');

-- AlterTable
ALTER TABLE "Images" DROP COLUMN "status",
ADD COLUMN     "status" "status" NOT NULL;

-- AlterTable
ALTER TABLE "TrainingModels" DROP COLUMN "training_status",
ADD COLUMN     "training_status" "status" NOT NULL;

-- AlterTable
ALTER TABLE "Transactions" DROP COLUMN "status",
ADD COLUMN     "status" "status" NOT NULL;

-- DropEnum
DROP TYPE "Status";
