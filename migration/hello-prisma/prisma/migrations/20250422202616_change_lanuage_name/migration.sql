/*
  Warnings:

  - The values [EN,SP,DE,IT,NL] on the enum `language` will be removed. If these variants are still used in the database, this will fail.

*/
-- AlterEnum
BEGIN;
CREATE TYPE "language_new" AS ENUM ('English', 'Spanish', 'German', 'Italian', 'Dutch');
ALTER TABLE "UserSettings" ALTER COLUMN "language" DROP DEFAULT;
ALTER TABLE "UserSettings" ALTER COLUMN "language" TYPE "language_new" USING ("language"::text::"language_new");
ALTER TYPE "language" RENAME TO "language_old";
ALTER TYPE "language_new" RENAME TO "language";
DROP TYPE "language_old";
ALTER TABLE "UserSettings" ALTER COLUMN "language" SET DEFAULT 'English';
COMMIT;

-- AlterTable
ALTER TABLE "UserSettings" ALTER COLUMN "language" SET DEFAULT 'English';
