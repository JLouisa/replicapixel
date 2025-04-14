# users:

Loco Generated

# Training Models:

cargo loco generate scaffold training_models pid:uuid_col! users:references name:string! age:tiny_int! sex:string! ethnicity:string! type_model:string! eye_color:string! bald:bool! steps:int! create_mask:bool! is_style:bool! trigger_word:string! tensor_path:string thumbnail:string training_status:string! fal_output:json! output_images:json fal_ai_request_id:string s3_key:string! is_verified:bool! --api

# Images:

cargo loco generate scaffold images pid:uuid_col! users:references training_models:references user_prompt:string! sys_prompt:string! num_inference_steps:int! content_type:string! status:string! image_size:string! fal_ai_request_id:string width:int height:int image_url:string image_url_s3:string is_favorite:bool! deleted_at:tstz --api

<!-- ========================================== -->

# Packs:

cargo loco generate scaffold packs pid:uuid_col! name:string! description:string pack_prompts:string! credits:int! amount:int! image_url:string --api

# Subscriptions:

cargo loco generate scaffold subscriptions pid:uuid_col! users:references plan:string! payment_id:string order_id:string --api

# UserCredits:

cargo loco generate scaffold user_credits pid:uuid_col! users:references credit_amount:int! model_amount:int! --api

# Transactions:

cargo loco generate scaffold transactions pid:uuid_col! users:references credit_amount:int! model_amount:int! currency:string! payment_id:string order_id:string plan:string! status:string! --api

## Add more columns

cargo loco g migration AddSexToTrainings sex:string!
cargo loco g migration AddUserToTrainings users:references
cargo loco g migration RemoveZip_urlFromTrainings zip_url:string!
cargo loco g migration AddFavoriteAndDeletedToImages is_favorite:bool! is_deleted:bool!
cargo loco g migration AddModel_amountToUser_credits model_amount:int!
cargo loco g migration AddPack_idToImages packs:references

## Script:

You can apply it:

$ cargo loco db migrate

# Generate back entities (Rust code) from it:

$ bunx prisma migrate dev --name FixEnumTypeMappingRest
$ cargo loco db entities

### Prisma:

bunx prisma migrate dev --name <Name the migration>
bunx prisma migrate reset --force
bunx prisma db seed

#

<html>
  #
  <body>
    # Dear {{name}}, # Welcome to Loco! You can now log in to your account. # Before you get
    started, please verify your account by clicking the link below: #
    <a href="{{domain}}/api/auth/verify/{{verifyToken}}"> # Verify Your Account # </a> #
    <p>Best regards,<br />The Loco Team</p>
    #
  </body>

</html>
  #
