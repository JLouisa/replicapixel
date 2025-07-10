-- CreateEnum
CREATE TYPE "plan_category" AS ENUM ('Main', 'Sub');

-- AlterEnum
-- This migration adds more than one value to an enum.
-- With PostgreSQL versions 11 and earlier, this is not possible
-- in a single migration. This can be worked around by creating
-- multiple migrations, each migration adding only one value to
-- the enum.


ALTER TYPE "plan_names" ADD VALUE 'BasicPlus';
ALTER TYPE "plan_names" ADD VALUE 'PremiumPlus';
ALTER TYPE "plan_names" ADD VALUE 'MaxPlus';

-- AlterTable
ALTER TABLE "Plans" ADD COLUMN     "category" "plan_category" NOT NULL DEFAULT 'Sub';
