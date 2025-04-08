/*
  Warnings:

  - Added the required column `plan_name` to the `Plans` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "plan_names" AS ENUM ('Basic', 'Premium', 'Max');

-- AlterTable
ALTER TABLE "Plans" ADD COLUMN     "plan_name" "plan_names" NOT NULL;
