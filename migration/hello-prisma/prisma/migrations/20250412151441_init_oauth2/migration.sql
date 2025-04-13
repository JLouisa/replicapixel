-- CreateTable
CREATE TABLE "o_auth2_sessions" (
    "id" SERIAL NOT NULL,
    "user_id" INTEGER NOT NULL,
    "session_id" VARCHAR(255) NOT NULL,
    "expires_at" TIMESTAMPTZ NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "o_auth2_sessions_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "idx-oauth2-session_id" ON "o_auth2_sessions"("session_id");

-- CreateIndex
CREATE INDEX "idx-images-created_at" ON "Images"("created_at");

-- CreateIndex
CREATE INDEX "idx-images-updated_at" ON "Images"("updated_at");

-- AddForeignKey
ALTER TABLE "o_auth2_sessions" ADD CONSTRAINT "o_auth2_sessions_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "Users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
