import { P as defineComponent, a8 as useAppStore, Z as onMounted, H as api, M as createElementBlock, J as createBaseVNode, O as createVNode, af as withCtx, o as VIcon, d as VCard, c as VBtn, a1 as ref, $ as openBlock, N as createTextVNode, h as VCardText, E as VTextarea } from "./main-D22gLLWp.js";
const _hoisted_1 = { class: "d-flex align-center mb-2" };
const _hoisted_2 = { class: "d-flex justify-end mt-4" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsAgentsMdPage",
  setup(__props) {
    const appStore = useAppStore();
    const content = ref("");
    const loading = ref(false);
    const saving = ref(false);
    onMounted(async () => {
      loading.value = true;
      try {
        content.value = await api.getAgentsMd();
      } catch {
        appStore.showMessage("Failed to load AGENTS.md", "error");
      } finally {
        loading.value = false;
      }
    });
    async function handleSave() {
      saving.value = true;
      try {
        await api.updateAgentsMd(content.value);
        appStore.showMessage("AGENTS.md saved successfully!", "success");
      } catch {
        appStore.showMessage("Failed to save AGENTS.md", "error");
      } finally {
        saving.value = false;
      }
    }
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createVNode(VIcon, {
            color: "primary",
            class: "mr-2"
          }, {
            default: withCtx(() => [..._cache[1] || (_cache[1] = [
              createTextVNode("mdi-text", -1)
            ])]),
            _: 1
          }),
          _cache[2] || (_cache[2] = createBaseVNode("h2", { class: "text-h5 font-weight-bold" }, "AGENTS.md", -1))
        ]),
        _cache[5] || (_cache[5] = createBaseVNode("p", { class: "text-body-2 text-medium-emphasis mb-4" }, " Configure the default AGENTS.md instructions that will be used for all AI interactions. ", -1)),
        createVNode(VCard, null, {
          default: withCtx(() => [
            createVNode(VCardText, null, {
              default: withCtx(() => [
                createVNode(VTextarea, {
                  modelValue: content.value,
                  "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => content.value = $event),
                  variant: "outlined",
                  rows: "10",
                  placeholder: "Enter AGENTS.md content...",
                  loading: loading.value
                }, null, 8, ["modelValue", "loading"])
              ]),
              _: 1
            })
          ]),
          _: 1
        }),
        createBaseVNode("div", _hoisted_2, [
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            loading: saving.value,
            onClick: handleSave
          }, {
            default: withCtx(() => [
              createVNode(VIcon, { start: "" }, {
                default: withCtx(() => [..._cache[3] || (_cache[3] = [
                  createTextVNode("mdi-content-save", -1)
                ])]),
                _: 1
              }),
              _cache[4] || (_cache[4] = createTextVNode(" Save ", -1))
            ]),
            _: 1
          }, 8, ["loading"])
        ])
      ]);
    };
  }
});
export {
  _sfc_main as default
};
