/*
  Warnings:

  - Added the required column `cta` to the `Plans` table without a default value. This is not possible if the table is not empty.
  - Added the required column `subtitle` to the `Plans` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Plans" ADD COLUMN     "cta" VARCHAR(255) NOT NULL,
ADD COLUMN     "features" TEXT[],
ADD COLUMN     "is_popular" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "subtitle" VARCHAR(255) NOT NULL;
