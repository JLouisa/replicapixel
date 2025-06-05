-- CreateEnum
CREATE TYPE "account" AS ENUM ('Website', 'Google', 'Github');

-- AlterTable
ALTER TABLE "Users" ADD COLUMN     "account" "account" NOT NULL DEFAULT 'Website';
