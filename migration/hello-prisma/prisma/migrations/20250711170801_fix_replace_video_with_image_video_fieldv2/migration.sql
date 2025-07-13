/*
  Warnings:

  - You are about to drop the column `image_s3_key` on the `Videos` table. All the data in the column will be lost.
  - You are about to drop the column `image_url_fal` on the `Videos` table. All the data in the column will be lost.
  - Added the required column `video_s3_key` to the `Videos` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Videos" DROP COLUMN "image_s3_key",
DROP COLUMN "image_url_fal",
ADD COLUMN     "video_s3_key" VARCHAR(255) NOT NULL,
ADD COLUMN     "video_url_fal" TEXT;
