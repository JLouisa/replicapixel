/*
  Warnings:

  - You are about to drop the column `thumbnail` on the `Videos` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Videos" DROP COLUMN "thumbnail",
ADD COLUMN     "thumbnail_s3_key" VARCHAR(255);
