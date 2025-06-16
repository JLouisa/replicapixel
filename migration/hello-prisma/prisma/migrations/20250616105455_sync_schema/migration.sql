-- CreateEnum
CREATE TYPE "role" AS ENUM ('ADMIN', 'USER');

-- AlterTable
ALTER TABLE "Users" ADD COLUMN     "role" "role" NOT NULL DEFAULT 'USER';

-- CreateTable
CREATE TABLE "seaql_migrations" (
    "version" VARCHAR NOT NULL,
    "applied_at" BIGINT NOT NULL,

    CONSTRAINT "seaql_migrations_pkey" PRIMARY KEY ("version")
);
