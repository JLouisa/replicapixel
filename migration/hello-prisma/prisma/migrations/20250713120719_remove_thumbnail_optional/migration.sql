/*
  Warnings:

  - Made the column `thumbnail_s3_key` on table `Videos` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "Videos" ALTER COLUMN "thumbnail_s3_key" SET NOT NULL;
