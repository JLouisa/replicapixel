-- CreateEnum
CREATE TYPE "Sex" AS ENUM ('Male', 'Female');

-- CreateEnum
CREATE TYPE "Ethnicity" AS ENUM ('White', 'Black', 'Pacific', 'Hispanic', 'Asian', 'SouthEastAsian', 'SouthAsian', 'MiddleEastern');

-- CreateEnum
CREATE TYPE "BasedOn" AS ENUM ('RealPerson', 'CreateInfluencerAI');

-- CreateEnum
CREATE TYPE "EyeColor" AS ENUM ('Brown', 'Blue', 'Green', 'Grey', 'Hazel', 'Red');

-- CreateTable
CREATE TABLE "Users" (
    "id" SERIAL NOT NULL,
    "pid" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "apiKey" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "resetToken" TEXT,
    "resetSentAt" TIMESTAMPTZ,
    "emailVerificationToken" TEXT,
    "emailVerificationSentAt" TIMESTAMPTZ,
    "emailVerifiedAt" TIMESTAMPTZ,
    "magicLinkToken" TEXT,
    "magicLinkExpiration" TIMESTAMPTZ,

    CONSTRAINT "Users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "TrainingModels" (
    "id" SERIAL NOT NULL,
    "pid" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "age" TEXT NOT NULL,
    "sex" "Sex" NOT NULL,
    "ethnicity" "Ethnicity" NOT NULL,
    "basedOn" "BasedOn" NOT NULL,
    "eyeColor" "EyeColor",
    "bald" BOOLEAN NOT NULL,
    "steps" INTEGER NOT NULL,
    "createMask" BOOLEAN NOT NULL,
    "isStyle" BOOLEAN NOT NULL,
    "triggerWord" TEXT NOT NULL,
    "tensorPath" TEXT,
    "thumbnail" TEXT,
    "trainingStatus" TEXT NOT NULL,
    "falOutput" JSONB,
    "trainingImages" JSONB,
    "falAiRequestId" TEXT,
    "s3Key" TEXT NOT NULL,
    "isVerified" BOOLEAN NOT NULL,

    CONSTRAINT "TrainingModels_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Users_pid_key" ON "Users"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "Users_email_key" ON "Users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "Users_apiKey_key" ON "Users"("apiKey");

-- CreateIndex
CREATE INDEX "idx-users-pid" ON "Users"("pid");

-- CreateIndex
CREATE INDEX "idx-users-email" ON "Users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "TrainingModels_pid_key" ON "TrainingModels"("pid");

-- CreateIndex
CREATE UNIQUE INDEX "TrainingModels_userId_key" ON "TrainingModels"("userId");

-- CreateIndex
CREATE INDEX "idx-training_models-pid" ON "TrainingModels"("pid");

-- AddForeignKey
ALTER TABLE "TrainingModels" ADD CONSTRAINT "TrainingModels_userId_fkey" FOREIGN KEY ("userId") REFERENCES "Users"("pid") ON DELETE CASCADE ON UPDATE CASCADE;
