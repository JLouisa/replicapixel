/*
  Warnings:

  - You are about to drop the column `name` on the `Packs` table. All the data in the column will be lost.
  - Added the required column `title` to the `Packs` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "language" AS ENUM ('EN', 'SP', 'DE', 'IT', 'NL');

-- CreateEnum
CREATE TYPE "notification" AS ENUM ('Message', 'System_update', 'Promotion');

-- CreateEnum
CREATE TYPE "theme_preference" AS ENUM ('Light', 'Dark', 'System');

-- AlterTable
ALTER TABLE "Packs" DROP COLUMN "name",
ADD COLUMN     "title" VARCHAR(255) NOT NULL;

-- CreateTable
CREATE TABLE "Notification" (
    "id" SERIAL NOT NULL,
    "pid" UUID NOT NULL,
    "user_id" INTEGER NOT NULL,
    "message" TEXT NOT NULL,
    "read" BOOLEAN NOT NULL DEFAULT false,
    "link" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "type" "notification" NOT NULL DEFAULT 'Message',

    CONSTRAINT "Notification_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "UserSettings" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "enable_notification_email" BOOLEAN NOT NULL DEFAULT true,
    "enable_marketing_email" BOOLEAN NOT NULL DEFAULT true,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "language" "language" NOT NULL DEFAULT 'EN',
    "theme" "theme_preference" NOT NULL DEFAULT 'System',

    CONSTRAINT "UserSettings_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Notification_pid_key" ON "Notification"("pid");

-- CreateIndex
CREATE INDEX "Notification_user_id_idx" ON "Notification"("user_id");

-- CreateIndex
CREATE INDEX "Notification_user_id_read_idx" ON "Notification"("user_id", "read");

-- CreateIndex
CREATE UNIQUE INDEX "UserSettings_user_id_key" ON "UserSettings"("user_id");

-- CreateIndex
CREATE INDEX "UserSettings_user_id_idx" ON "UserSettings"("user_id");

-- AddForeignKey
ALTER TABLE "Notification" ADD CONSTRAINT "Notification_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "UserSettings" ADD CONSTRAINT "UserSettings_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
