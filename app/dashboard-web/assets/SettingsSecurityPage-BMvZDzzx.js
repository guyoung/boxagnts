import { R as defineComponent, a9 as useAppStore, O as createElementBlock, L as createBaseVNode, Q as createVNode, ag as withCtx, q as VIcon, e as VCard, d as VBtn, a1 as openBlock, P as createTextVNode, j as VCardTitle, i as VCardText, r as VList, F as Fragment, a3 as renderList, M as createBlock, v as VListItemTitle, a6 as toDisplayString, t as VListItem, a8 as unref, N as createCommentVNode, y as VRow, m as VCol, G as VTextField, ai as withKeys, a2 as ref } from "./index-CD7sFTTo.js";
import { u as useSettingsStore } from "./settings-jnLiHyJO.js";
const _hoisted_1 = { class: "d-flex align-center mb-2" };
const _hoisted_2 = { class: "d-flex justify-end mt-4" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsSecurityPage",
  setup(__props) {
    const settings = useSettingsStore();
    const appStore = useAppStore();
    const newOutboundHost = ref("");
    function handleSave() {
      appStore.showMessage("Security settings saved locally!", "success");
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
                          placeholder: "api.example.com",
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
            onClick: handleSave
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
          })
        ])
      ]);
    };
  }
});
export {
  _sfc_main as default
};
