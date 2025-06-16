-- CreateEnum
CREATE TYPE "role" AS ENUM ('Admin', 'User');

-- AlterTable
ALTER TABLE "Users" ADD COLUMN     "role" "role" NOT NULL DEFAULT 'User';
