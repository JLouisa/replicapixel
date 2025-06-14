-- CreateTable
CREATE TABLE "Plans_translations" (
    "id" SERIAL NOT NULL,
    "plan_id" INTEGER NOT NULL,
    "language" "language" NOT NULL,
    "name" TEXT NOT NULL,
    "subtitle" VARCHAR(255) NOT NULL,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "cta" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Plans_translations_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Packs_translations" (
    "id" SERIAL NOT NULL,
    "pack_id" INTEGER NOT NULL,
    "language" "language" NOT NULL,
    "title" VARCHAR(255) NOT NULL,
    "short_description" VARCHAR(255) NOT NULL,
    "full_description" TEXT NOT NULL,
    "features" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Packs_translations_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "Plans_translations_plan_id_idx" ON "Plans_translations"("plan_id");

-- CreateIndex
CREATE UNIQUE INDEX "Plans_translations_plan_id_language_key" ON "Plans_translations"("plan_id", "language");

-- CreateIndex
CREATE INDEX "Packs_translations_pack_id_idx" ON "Packs_translations"("pack_id");

-- CreateIndex
CREATE UNIQUE INDEX "Packs_translations_pack_id_language_key" ON "Packs_translations"("pack_id", "language");

-- AddForeignKey
ALTER TABLE "Plans_translations" ADD CONSTRAINT "Plans_translations_plan_id_fkey" FOREIGN KEY ("plan_id") REFERENCES "Plans"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Packs_translations" ADD CONSTRAINT "Packs_translations_pack_id_fkey" FOREIGN KEY ("pack_id") REFERENCES "Packs"("id") ON DELETE CASCADE ON UPDATE CASCADE;
