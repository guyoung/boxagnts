import { T as defineStore, a4 as ref } from "./main-gWZPyuWK.js";
const useSettingsStore = defineStore("settings", () => {
  const settings = ref({
    system_prompt: "",
    default_model: "",
    default_provider: "",
    allowed_outbound_hosts: []
  });
  return {
    settings
  };
});
export {
  useSettingsStore as u
};
