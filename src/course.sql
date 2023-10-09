-- ----------------------------
-- Table structure for course
-- ----------------------------
DROP TABLE IF EXISTS "public"."course";
CREATE TABLE "public"."course" (
                                   "id" SERIAL PRIMARY KEY,
                                   "teacher_id" int4 NOT NULL,
                                   "name" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
                                   "time" date DEFAULT now()
)
;

-- ----------------------------
-- Records of course
-- ----------------------------
-- INSERT INTO "public"."course" VALUES (1, 11, 'cml', '2023-10-09');
-- INSERT INTO "public"."course" VALUES (2, 22, 'cc', '2023-10-09');
-- INSERT INTO "public"."course" VALUES (3, 33, 'mm', '2023-10-09');

-- ----------------------------
-- Primary Key structure for table course
-- ----------------------------
-- ALTER TABLE "public"."course" ADD CONSTRAINT "course_pkey" PRIMARY KEY ("id");
