type CoursePageWithUserData = {
  page: Page
  instance: CourseInstance | null
  settings: UserCourseSettings | null
  was_redirected: boolean
  is_test_mode: boolean
}
