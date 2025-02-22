import { RoleDomain, RoleInfo, RoleQuery, RoleUser, UserRole } from "../../shared-module/bindings"
import { isRoleUser } from "../../shared-module/bindings.guard"
import { isArray, validateResponse } from "../../shared-module/utils/fetching"
import { mainFrontendClient } from "../mainFrontendClient"

export const fetchRoles = async (query: RoleQuery): Promise<Array<RoleUser>> => {
  const response = await mainFrontendClient.get(`/roles`, {
    params: query,
    responseType: "json",
  })
  return validateResponse(response, isArray(isRoleUser))
}

export const giveRole = async (
  email: string,
  role: UserRole,
  domain: RoleDomain,
): Promise<void> => {
  const data: RoleInfo = {
    email,
    role,
    domain,
  }
  await mainFrontendClient.post(`/roles/add`, data, { responseType: "json" })
}

export const removeRole = async (
  email: string,
  role: UserRole,
  domain: RoleDomain,
): Promise<void> => {
  const data: RoleInfo = {
    email,
    role,
    domain,
  }
  await mainFrontendClient.post(`/roles/remove`, data, { responseType: "json" })
}
