import { S as defineComponent, ab as useAppStore, a1 as onMounted, K as api, P as createElementBlock, M as createBaseVNode, R as createVNode, ai as withCtx, r as VIcon, f as VCard, e as VBtn, a4 as ref, a2 as openBlock, Q as createTextVNode, k as VCardTitle, j as VCardText, s as VList, F as Fragment, a5 as renderList, N as createBlock, v as VListItemTitle, a8 as toDisplayString, t as VListItem, aa as unref, O as createCommentVNode, z as VRow, n as VCol, H as VTextField, ak as withKeys } from "./main-BSD2YpbL.js";
import { u as useSettingsStore } from "./settings-RFBLOOFr.js";
const _hoisted_1 = { class: "d-flex align-center mb-2" };
const _hoisted_2 = { class: "d-flex justify-end mt-4" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsSecurityPage",
  setup(__props) {
    const settings = useSettingsStore();
    const appStore = useAppStore();
    const newOutboundHost = ref("");
    const saving = ref(false);
    const loading = ref(false);
    onMounted(async () => {
      loading.value = true;
      try {
        const hosts = await api.getAllowedOutboundHosts();
        settings.settings.allowed_outbound_hosts = hosts;
      } catch {
        appStore.showMessage("Failed to load allowed outbound hosts", "error");
      } finally {
        loading.value = false;
      }
    });
    async function handleSave() {
      saving.value = true;
      try {
        await api.updateAllowedOutboundHosts(settings.settings.allowed_outbound_hosts || []);
        appStore.showMessage("Security settings saved!", "success");
      } catch {
        appStore.showMessage("Failed to save security settings", "error");
      } finally {
        saving.value = false;
      }
    }
    function addOutboundHost() {
      const host = newOutboundHost.value.trim();
      if (!host) return;
      if (!settings.settings.allowed_outbound_hosts) {
        settings.settings.allowed_outbound_hosts = [];
      }
      if (!settings.settings.allowed_outbound_hosts.includes(host)) {
        settings.settings.allowed_outbound_hosts.push(host);
      }
      newOutboundHost.value = "";
    }
    function deleteOutboundHost(idx) {
      if (!settings.settings.allowed_outbound_hosts) return;
      settings.settings.allowed_outbound_hosts.splice(idx, 1);
    }
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createVNode(VIcon, {
            color: "primary",
            class: "mr-2"
          }, {
            default: withCtx(() => [..._cache[1] || (_cache[1] = [
              createTextVNode("mdi-security", -1)
            ])]),
            _: 1
          }),
          _cache[2] || (_cache[2] = createBaseVNode("h2", { class: "text-h5 font-weight-bold" }, "Security Settings", -1))
        ]),
        _cache[10] || (_cache[10] = createBaseVNode("p", { class: "text-body-2 text-medium-emphasis mb-4" }, " Manage allowed outbound hosts for network access control. ", -1)),
        createVNode(VCard, null, {
          default: withCtx(() => [
            createVNode(VCardTitle, { class: "d-flex align-center" }, {
              default: withCtx(() => [
                createVNode(VIcon, { start: "" }, {
                  default: withCtx(() => [..._cache[3] || (_cache[3] = [
                    createTextVNode("mdi-server-network", -1)
                  ])]),
                  _: 1
                }),
                _cache[4] || (_cache[4] = createTextVNode(" Allowed Outbound Hosts ", -1))
              ]),
              _: 1
            }),
            createVNode(VCardText, null, {
              default: withCtx(() => [
                createVNode(VList, {
                  density: "compact",
                  class: "mb-4"
                }, {
                  default: withCtx(() => [
                    (openBlock(true), createElementBlock(Fragment, null, renderList(unref(settings).settings.allowed_outbound_hosts, (host, idx) => {
                      return openBlock(), createBlock(VListItem, {
                        key: idx,
                        rounded: "lg"
                      }, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, null, {
                            default: withCtx(() => [..._cache[5] || (_cache[5] = [
                              createTextVNode("mdi-server", -1)
                            ])]),
                            _: 1
                          })
                        ]),
                        append: withCtx(() => [
                          createVNode(VBtn, {
                            icon: "mdi-delete",
                            variant: "text",
                            color: "error",
                            onClick: ($event) => deleteOutboundHost(idx)
                          }, null, 8, ["onClick"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VListItemTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(host), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024);
                    }), 128)),
                    !unref(settings).settings.allowed_outbound_hosts || unref(settings).settings.allowed_outbound_hosts.length === 0 ? (openBlock(), createBlock(VListItem, { key: 0 }, {
                      default: withCtx(() => [
                        createVNode(VListItemTitle, { class: "text-medium-emphasis" }, {
                          default: withCtx(() => [..._cache[6] || (_cache[6] = [
                            createTextVNode("No outbound hosts configured", -1)
                          ])]),
                          _: 1
                        })
                      ]),
                      _: 1
                    })) : createCommentVNode("", true)
                  ]),
                  _: 1
                }),
                createVNode(VRow, {
                  dense: "",
                  align: "center"
                }, {
                  default: withCtx(() => [
                    createVNode(VCol, { cols: "10" }, {
                      default: withCtx(() => [
                        createVNode(VTextField, {
                          modelValue: newOutboundHost.value,
                          "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => newOutboundHost.value = $event),
                          label: "Add Host",
                          variant: "outlined",
                          placeholder: "https://api.example.com",
                          onKeyup: withKeys(addOutboundHost, ["enter"])
                        }, null, 8, ["modelValue"])
                      ]),
                      _: 1
                    }),
                    createVNode(VCol, { cols: "2" }, {
                      default: withCtx(() => [
                        createVNode(VBtn, {
                          color: "primary",
                          block: "",
                          onClick: addOutboundHost,
                          disabled: !newOutboundHost.value.trim()
                        }, {
                          default: withCtx(() => [..._cache[7] || (_cache[7] = [
                            createTextVNode(" Add ", -1)
                          ])]),
                          _: 1
                        }, 8, ["disabled"])
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                })
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
            onClick: handleSave,
            loading: saving.value
          }, {
            default: withCtx(() => [
              createVNode(VIcon, { start: "" }, {
                default: withCtx(() => [..._cache[8] || (_cache[8] = [
                  createTextVNode("mdi-content-save", -1)
                ])]),
                _: 1
              }),
              _cache[9] || (_cache[9] = createTextVNode(" Save ", -1))
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
