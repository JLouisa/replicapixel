/*
  Warnings:

  - You are about to alter the column `email` on the `Users` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - You are about to alter the column `api_key` on the `Users` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(255)`.
  - Changed the type of `content_type` on the `Images` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `image_size` on the `Images` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "Emotion" AS ENUM ('Neutral', 'Anger', 'Disgust', 'Fear', 'Happy', 'Sad', 'Surprise');

-- CreateEnum
CREATE TYPE "ImageSize" AS ENUM ('Square', 'SquareHD', 'Portrait43', 'Portrait169', 'Landscape43', 'Landscape169');

-- CreateEnum
CREATE TYPE "ImageFormat" AS ENUM ('Png', 'Jpeg', 'Zip');

-- AlterTable
ALTER TABLE "Images" DROP COLUMN "content_type",
ADD COLUMN     "content_type" "ImageFormat" NOT NULL,
DROP COLUMN "image_size",
ADD COLUMN     "image_size" "ImageSize" NOT NULL;

-- AlterTable
ALTER TABLE "TrainingModels" ALTER COLUMN "bald" SET DEFAULT false;

-- AlterTable
ALTER TABLE "Users" ALTER COLUMN "email" SET DATA TYPE VARCHAR(255),
ALTER COLUMN "api_key" SET DATA TYPE VARCHAR(255);
