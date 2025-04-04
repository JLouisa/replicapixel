/*
  Warnings:

  - Changed the type of `content_type` on the `Images` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `status` on the `Images` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `image_size` on the `Images` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `sex` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `ethnicity` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `eye_color` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `training_status` on the `TrainingModels` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `status` on the `Transactions` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "status" AS ENUM ('Pending', 'Processing', 'Training', 'Completed', 'Failed', 'Cancelled');

-- CreateEnum
CREATE TYPE "sex" AS ENUM ('Male', 'Female');

-- CreateEnum
CREATE TYPE "ethnicity" AS ENUM ('White', 'Black', 'Pacific', 'Hispanic', 'Asian', 'SouthEastAsian', 'SouthAsian', 'MiddleEastern');

-- CreateEnum
CREATE TYPE "eyeColor" AS ENUM ('Brown', 'Blue', 'Green', 'Grey', 'Hazel', 'Red');

-- CreateEnum
CREATE TYPE "emotion" AS ENUM ('Neutral', 'Anger', 'Disgust', 'Fear', 'Happy', 'Sad', 'Surprise');

-- CreateEnum
CREATE TYPE "imageSize" AS ENUM ('Square', 'SquareHD', 'Portrait43', 'Portrait169', 'Landscape43', 'Landscape169');

-- CreateEnum
CREATE TYPE "imageFormat" AS ENUM ('Png', 'Jpeg', 'Zip');

-- AlterTable
ALTER TABLE "Images" DROP COLUMN "content_type",
ADD COLUMN     "content_type" "imageFormat" NOT NULL,
DROP COLUMN "status",
ADD COLUMN     "status" "status" NOT NULL,
DROP COLUMN "image_size",
ADD COLUMN     "image_size" "imageSize" NOT NULL;

-- AlterTable
ALTER TABLE "TrainingModels" DROP COLUMN "sex",
ADD COLUMN     "sex" "sex" NOT NULL,
DROP COLUMN "ethnicity",
ADD COLUMN     "ethnicity" "ethnicity" NOT NULL,
DROP COLUMN "eye_color",
ADD COLUMN     "eye_color" "eyeColor" NOT NULL,
DROP COLUMN "training_status",
ADD COLUMN     "training_status" "status" NOT NULL;

-- AlterTable
ALTER TABLE "Transactions" DROP COLUMN "status",
ADD COLUMN     "status" "status" NOT NULL;

-- DropEnum
DROP TYPE "Emotion";

-- DropEnum
DROP TYPE "Ethnicity";

-- DropEnum
DROP TYPE "EyeColor";

-- DropEnum
DROP TYPE "ImageFormat";

-- DropEnum
DROP TYPE "ImageSize";

-- DropEnum
DROP TYPE "Sex";

-- DropEnum
DROP TYPE "Status";
