import { Q as defineStore, H as api, P as defineComponent, $ as openBlock, M as createElementBlock, F as Fragment, a2 as renderList, O as createVNode, q as VListItem, af as withCtx, o as VIcon, ai as withModifiers, N as createTextVNode, a5 as toDisplayString, n as VExpandTransition, ag as withDirectives, ad as vShow, p as VList, K as createBlock, a3 as resolveComponent, _ as _export_sfc, a8 as useAppStore, Z as onMounted, J as createBaseVNode, c as VBtn, a7 as unref, L as createCommentVNode, w as VRow, l as VDialog, a1 as ref, d as VCard, X as normalizeClass, f as VCardItem, i as VCardTitle, g as VCardSubtitle, b as VAvatar, h as VCardText, z as VSpacer, j as VChip, e as VCardActions, A as VSwitch, k as VCol, y as VSkeletonLoader, D as VTextField, E as VTextarea, t as VMenu, m as VDivider, U as mergeProps, x as VSelect, I as computed, v as VProgressLinear } from "./main-D22gLLWp.js";
import { u as useCrudOperations } from "./baseCrud-jA2AHr9e.js";
const useSiteStore = defineStore("sites", () => {
  const crud = useCrudOperations(
    {
      fetchAll: () => api.getSites(),
      create: (data) => api.createSite(data),
      update: (id, data) => api.updateSite(id, data),
      remove: (id) => api.deleteSite(id)
    },
    "sites"
  );
  return {
    sites: crud.items,
    loading: crud.loading,
    fetchSites: crud.fetch,
    addSite: crud.add,
    updateSite: crud.update,
    removeSite: crud.remove
  };
});
const _sfc_main$1 = /* @__PURE__ */ defineComponent({
  __name: "FolderTreeItem",
  props: {
    nodes: {},
    currentPath: {},
    expandedPaths: {}
  },
  emits: ["select", "toggle"],
  setup(__props, { emit: __emit }) {
    const props = __props;
    const emit = __emit;
    function isExpanded(path) {
      return props.expandedPaths.has(path);
    }
    function toggleExpand(path) {
      emit("toggle", path);
    }
    return (_ctx, _cache) => {
      const _component_FolderTreeItem = resolveComponent("FolderTreeItem", true);
      return openBlock(true), createElementBlock(Fragment, null, renderList(__props.nodes, (node) => {
        return openBlock(), createElementBlock(Fragment, {
          key: node.path
        }, [
          node.children.length > 0 ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
            createVNode(VListItem, {
              title: node.name,
              "prepend-icon": "mdi-folder",
              active: __props.currentPath === node.path,
              color: __props.currentPath === node.path ? "primary" : void 0,
              density: "compact",
              onClick: ($event) => _ctx.$emit("select", node.path)
            }, {
              append: withCtx(() => [
                createVNode(VIcon, {
                  size: "20",
                  class: "folder-expand-icon",
                  onClick: withModifiers(($event) => toggleExpand(node.path), ["stop"])
                }, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(isExpanded(node.path) ? "mdi-chevron-up" : "mdi-chevron-down"), 1)
                  ]),
                  _: 2
                }, 1032, ["onClick"])
              ]),
              _: 2
            }, 1032, ["title", "active", "color", "onClick"]),
            createVNode(VExpandTransition, null, {
              default: withCtx(() => [
                withDirectives(createVNode(VList, {
                  density: "compact",
                  class: "pl-4"
                }, {
                  default: withCtx(() => [
                    createVNode(_component_FolderTreeItem, {
                      nodes: node.children,
                      "current-path": __props.currentPath,
                      "expanded-paths": __props.expandedPaths,
                      onSelect: _cache[0] || (_cache[0] = (p) => _ctx.$emit("select", p)),
                      onToggle: _cache[1] || (_cache[1] = (p) => _ctx.$emit("toggle", p))
                    }, null, 8, ["nodes", "current-path", "expanded-paths"])
                  ]),
                  _: 2
                }, 1536), [
                  [vShow, isExpanded(node.path)]
                ])
              ]),
              _: 2
            }, 1024)
          ], 64)) : (openBlock(), createBlock(VListItem, {
            key: 1,
            title: node.name,
            "prepend-icon": "mdi-folder-outline",
            active: __props.currentPath === node.path,
            color: __props.currentPath === node.path ? "primary" : void 0,
            density: "compact",
            onClick: ($event) => _ctx.$emit("select", node.path)
          }, null, 8, ["title", "active", "color", "onClick"]))
        ], 64);
      }), 128);
    };
  }
});
const FolderTreeItem = /* @__PURE__ */ _export_sfc(_sfc_main$1, [["__scopeId", "data-v-17608d93"]]);
const _hoisted_1 = { class: "sites-page" };
const _hoisted_2 = { class: "page-header d-flex align-center justify-space-between mb-6" };
const _hoisted_3 = { class: "d-flex align-center" };
const _hoisted_4 = { class: "header-icon-wrapper mr-3" };
const _hoisted_5 = {
  key: 0,
  class: "stats-bar d-flex align-center pa-4 mb-6 rounded-lg"
};
const _hoisted_6 = { class: "stat-item d-flex align-center" };
const _hoisted_7 = { class: "text-body-2 font-weight-medium" };
const _hoisted_8 = { class: "stat-item d-flex align-center" };
const _hoisted_9 = { class: "text-body-2 font-weight-medium" };
const _hoisted_10 = { class: "stat-item d-flex align-center" };
const _hoisted_11 = { class: "text-body-2 font-weight-medium" };
const _hoisted_12 = { class: "text-truncate" };
const _hoisted_13 = { class: "site-name-code" };
const _hoisted_14 = {
  key: 0,
  class: "site-description text-body-2 mb-3"
};
const _hoisted_15 = { class: "info-rows" };
const _hoisted_16 = { class: "info-row d-flex align-center" };
const _hoisted_17 = { class: "text-caption info-value" };
const _hoisted_18 = {
  key: 0,
  class: "info-row d-flex align-center"
};
const _hoisted_19 = { class: "text-caption info-value" };
const _hoisted_20 = {
  key: 1,
  class: "info-row d-flex align-center"
};
const _hoisted_21 = { class: "text-caption info-value" };
const _hoisted_22 = { class: "d-flex align-center gap-2 mt-3" };
const _hoisted_23 = {
  key: 2,
  class: "empty-state text-center py-16"
};
const _hoisted_24 = { class: "empty-icon-wrapper mb-4" };
const _hoisted_25 = { class: "text-h6 font-weight-bold" };
const _hoisted_26 = { class: "text-caption text-medium-emphasis" };
const _hoisted_27 = { class: "form-section mb-4" };
const _hoisted_28 = { class: "form-section-header d-flex align-center mb-3" };
const _hoisted_29 = { class: "form-section mb-4" };
const _hoisted_30 = { class: "form-section-header d-flex align-center mb-3" };
const _hoisted_31 = { class: "pa-2 d-flex align-center" };
const _hoisted_32 = { class: "form-section" };
const _hoisted_33 = { class: "form-section-header d-flex align-center mb-3" };
const _hoisted_34 = { class: "delete-confirm pa-4 rounded-lg mb-4" };
const _hoisted_35 = { class: "text-body-1 mb-1" };
const _hoisted_36 = { class: "text-error" };
const _hoisted_37 = { class: "text-caption text-medium-emphasis mt-2" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SitesPage",
  setup(__props) {
    const siteStore = useSiteStore();
    const appStore = useAppStore();
    const showDialog = ref(false);
    const editingSite = ref(null);
    const saving = ref(false);
    const deleteDialog = ref(false);
    const deleteTarget = ref(null);
    const deleting = ref(false);
    const folders = ref([]);
    const loadingFolders = ref(false);
    const pathMenuOpen = ref(false);
    const expandedPaths = ref(/* @__PURE__ */ new Set());
    const componentOptions = [
      "boxed_static_server_component.wasm"
    ];
    const defaultForm = () => ({
      name: "",
      title: "",
      description: "",
      path: "",
      entry_point: null,
      component: "boxed_static_server_component.wasm",
      enabled: true,
      enable_auth: null,
      auth_user: null,
      auth_pass: null
    });
    const form = ref(defaultForm());
    function buildTree(folderItems) {
      const root = [];
      const pathMap = /* @__PURE__ */ new Map();
      folderItems.forEach((item) => {
        const node = { name: item.name, path: item.path, children: [] };
        pathMap.set(item.path, node);
      });
      folderItems.forEach((item) => {
        const node = pathMap.get(item.path);
        const parts = item.path.split("/");
        if (parts.length === 1) {
          root.push(node);
        } else {
          const parentPath = parts.slice(0, -1).join("/");
          const parent = pathMap.get(parentPath);
          if (parent) {
            parent.children.push(node);
          } else {
            root.push(node);
          }
        }
      });
      return root;
    }
    const folderTree = computed(() => buildTree(folders.value));
    async function fetchFolders() {
      loadingFolders.value = true;
      try {
        folders.value = await api.getRootSubFolders();
      } catch (e) {
        console.error("Failed to fetch folders:", e);
        folders.value = [];
      } finally {
        loadingFolders.value = false;
      }
    }
    function openAddDialog() {
      editingSite.value = null;
      form.value = defaultForm();
      showDialog.value = true;
      resetExpandedPaths();
      fetchFolders();
    }
    function openEditDialog(site) {
      editingSite.value = site;
      form.value = {
        name: site.name,
        title: site.title,
        description: site.description,
        path: site.path,
        entry_point: site.entry_point,
        component: site.component,
        enabled: site.enabled,
        enable_auth: site.enable_auth,
        auth_user: site.auth_user,
        auth_pass: site.auth_pass
      };
      showDialog.value = true;
      resetExpandedPaths();
      fetchFolders();
    }
    function selectPath(path) {
      form.value.path = path;
    }
    function toggleExpand(path) {
      const next = new Set(expandedPaths.value);
      if (next.has(path)) {
        next.delete(path);
      } else {
        next.add(path);
      }
      expandedPaths.value = next;
    }
    function buildSiteUrl(site) {
      const entry = site.entry_point || "index.html";
      return `${window.location.origin}/sites/${site.name}/${entry}`;
    }
    function openSiteUrl(site) {
      window.open(buildSiteUrl(site), "_blank");
    }
    function resetExpandedPaths() {
      expandedPaths.value = /* @__PURE__ */ new Set();
    }
    async function handleSave() {
      saving.value = true;
      try {
        if (editingSite.value) {
          await siteStore.updateSite(editingSite.value.id, { ...form.value });
          appStore.showMessage("Site updated successfully", "success");
        } else {
          await siteStore.addSite({ ...form.value });
          appStore.showMessage("Site created successfully", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingSite.value = null;
      } catch {
        appStore.showMessage("Failed to save site", "error");
        siteStore.fetchSites();
      } finally {
        saving.value = false;
      }
    }
    function confirmRemove(site) {
      deleteTarget.value = site;
      deleteDialog.value = true;
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await siteStore.removeSite(deleteTarget.value.id);
        appStore.showMessage("Site deleted successfully", "success");
      } catch {
        appStore.showMessage("Failed to delete site", "error");
        siteStore.fetchSites();
      } finally {
        deleting.value = false;
        deleteDialog.value = false;
        deleteTarget.value = null;
      }
    }
    async function handleToggleEnabled(site, enabled) {
      site.enabled = enabled;
      try {
        await siteStore.updateSite(site.id, { enabled });
        appStore.showMessage(enabled ? "Site enabled" : "Site disabled", "success");
      } catch {
        site.enabled = !enabled;
        appStore.showMessage("Failed to update site", "error");
        siteStore.fetchSites();
      }
    }
    onMounted(() => {
      siteStore.fetchSites();
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1, [
        createBaseVNode("div", _hoisted_2, [
          createBaseVNode("div", _hoisted_3, [
            createBaseVNode("div", _hoisted_4, [
              createVNode(VIcon, {
                size: "28",
                color: "white"
              }, {
                default: withCtx(() => [..._cache[17] || (_cache[17] = [
                  createTextVNode("mdi-web", -1)
                ])]),
                _: 1
              })
            ]),
            _cache[18] || (_cache[18] = createBaseVNode("div", null, [
              createBaseVNode("h1", { class: "text-h4 font-weight-bold mb-0" }, "Sites"),
              createBaseVNode("p", { class: "text-body-2 text-medium-emphasis mt-1" }, "Manage your static website deployments")
            ], -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            "prepend-icon": "mdi-plus",
            variant: "elevated",
            class: "add-btn",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[19] || (_cache[19] = [
              createTextVNode(" Add Site ", -1)
            ])]),
            _: 1
          })
        ]),
        !unref(siteStore).loading && unref(siteStore).sites.length > 0 ? (openBlock(), createElementBlock("div", _hoisted_5, [
          createBaseVNode("div", _hoisted_6, [
            createVNode(VIcon, {
              size: "20",
              color: "primary",
              class: "mr-2"
            }, {
              default: withCtx(() => [..._cache[20] || (_cache[20] = [
                createTextVNode("mdi-web", -1)
              ])]),
              _: 1
            }),
            createBaseVNode("span", _hoisted_7, toDisplayString(unref(siteStore).sites.length) + " site" + toDisplayString(unref(siteStore).sites.length > 1 ? "s" : ""), 1)
          ]),
          createVNode(VDivider, {
            vertical: "",
            class: "mx-4"
          }),
          createBaseVNode("div", _hoisted_8, [
            createVNode(VIcon, {
              size: "20",
              color: "success",
              class: "mr-2"
            }, {
              default: withCtx(() => [..._cache[21] || (_cache[21] = [
                createTextVNode("mdi-check-circle", -1)
              ])]),
              _: 1
            }),
            createBaseVNode("span", _hoisted_9, toDisplayString(unref(siteStore).sites.filter((s) => s.enabled).length) + " enabled", 1)
          ]),
          createVNode(VDivider, {
            vertical: "",
            class: "mx-4"
          }),
          createBaseVNode("div", _hoisted_10, [
            createVNode(VIcon, {
              size: "20",
              color: "warning",
              class: "mr-2"
            }, {
              default: withCtx(() => [..._cache[22] || (_cache[22] = [
                createTextVNode("mdi-shield-lock", -1)
              ])]),
              _: 1
            }),
            createBaseVNode("span", _hoisted_11, toDisplayString(unref(siteStore).sites.filter((s) => s.enable_auth).length) + " with auth", 1)
          ])
        ])) : createCommentVNode("", true),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(siteStore).sites, (site, idx) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: site.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, {
                    class: normalizeClass(["site-card fill-height", { "site-disabled": !site.enabled }]),
                    elevation: "2"
                  }, {
                    default: withCtx(() => [
                      createBaseVNode("div", {
                        class: normalizeClass(["card-top-bar", site.enabled ? "bg-primary" : "bg-grey"])
                      }, null, 2),
                      createVNode(VCardItem, { class: "pb-0" }, {
                        prepend: withCtx(() => [
                          createVNode(VAvatar, {
                            color: site.enable_auth ? "warning" : "success",
                            size: "44",
                            variant: "tonal"
                          }, {
                            default: withCtx(() => [
                              createVNode(VIcon, {
                                color: site.enable_auth ? "warning" : "success",
                                size: "22"
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(site.enable_auth ? "mdi-shield-lock" : "mdi-web"), 1)
                                ]),
                                _: 2
                              }, 1032, ["color"])
                            ]),
                            _: 2
                          }, 1032, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, { class: "text-h6 d-flex align-center" }, {
                            default: withCtx(() => [
                              createBaseVNode("span", _hoisted_12, toDisplayString(site.title || site.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VCardSubtitle, { class: "text-caption" }, {
                            default: withCtx(() => [
                              createBaseVNode("code", _hoisted_13, toDisplayString(site.name), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, { class: "pt-2" }, {
                        default: withCtx(() => [
                          site.description ? (openBlock(), createElementBlock("div", _hoisted_14, toDisplayString(site.description), 1)) : createCommentVNode("", true),
                          createBaseVNode("div", _hoisted_15, [
                            createBaseVNode("div", _hoisted_16, [
                              createVNode(VIcon, {
                                size: "15",
                                color: "medium-emphasis",
                                class: "mr-2"
                              }, {
                                default: withCtx(() => [..._cache[23] || (_cache[23] = [
                                  createTextVNode("mdi-folder", -1)
                                ])]),
                                _: 1
                              }),
                              _cache[24] || (_cache[24] = createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Path", -1)),
                              createVNode(VSpacer),
                              createBaseVNode("code", _hoisted_17, toDisplayString(site.path), 1)
                            ]),
                            site.entry_point ? (openBlock(), createElementBlock("div", _hoisted_18, [
                              createVNode(VIcon, {
                                size: "15",
                                color: "medium-emphasis",
                                class: "mr-2"
                              }, {
                                default: withCtx(() => [..._cache[25] || (_cache[25] = [
                                  createTextVNode("mdi-file-code", -1)
                                ])]),
                                _: 1
                              }),
                              _cache[26] || (_cache[26] = createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Entry", -1)),
                              createVNode(VSpacer),
                              createBaseVNode("code", _hoisted_19, toDisplayString(site.entry_point), 1)
                            ])) : createCommentVNode("", true),
                            site.enable_auth && site.auth_user ? (openBlock(), createElementBlock("div", _hoisted_20, [
                              createVNode(VIcon, {
                                size: "15",
                                color: "medium-emphasis",
                                class: "mr-2"
                              }, {
                                default: withCtx(() => [..._cache[27] || (_cache[27] = [
                                  createTextVNode("mdi-account", -1)
                                ])]),
                                _: 1
                              }),
                              _cache[28] || (_cache[28] = createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Auth", -1)),
                              createVNode(VSpacer),
                              createBaseVNode("span", _hoisted_21, toDisplayString(site.auth_user), 1)
                            ])) : createCommentVNode("", true)
                          ]),
                          createBaseVNode("div", _hoisted_22, [
                            site.enable_auth ? (openBlock(), createBlock(VChip, {
                              key: 0,
                              color: "warning",
                              size: "x-small",
                              variant: "tonal"
                            }, {
                              default: withCtx(() => [
                                createVNode(VIcon, {
                                  start: "",
                                  size: "14"
                                }, {
                                  default: withCtx(() => [..._cache[29] || (_cache[29] = [
                                    createTextVNode("mdi-shield-lock", -1)
                                  ])]),
                                  _: 1
                                }),
                                _cache[30] || (_cache[30] = createTextVNode(" Auth Enabled ", -1))
                              ]),
                              _: 1
                            })) : (openBlock(), createBlock(VChip, {
                              key: 1,
                              color: "success",
                              size: "x-small",
                              variant: "tonal"
                            }, {
                              default: withCtx(() => [
                                createVNode(VIcon, {
                                  start: "",
                                  size: "14"
                                }, {
                                  default: withCtx(() => [..._cache[31] || (_cache[31] = [
                                    createTextVNode("mdi-earth", -1)
                                  ])]),
                                  _: 1
                                }),
                                _cache[32] || (_cache[32] = createTextVNode(" Public ", -1))
                              ]),
                              _: 1
                            })),
                            createVNode(VChip, {
                              color: site.enabled ? "success" : "grey",
                              size: "x-small",
                              variant: "tonal"
                            }, {
                              default: withCtx(() => [
                                createVNode(VIcon, {
                                  start: "",
                                  size: "14"
                                }, {
                                  default: withCtx(() => [
                                    createTextVNode(toDisplayString(site.enabled ? "mdi-check-circle" : "mdi-cancel"), 1)
                                  ]),
                                  _: 2
                                }, 1024),
                                createTextVNode(" " + toDisplayString(site.enabled ? "Active" : "Inactive"), 1)
                              ]),
                              _: 2
                            }, 1032, ["color"])
                          ])
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, { class: "px-4 pb-3 pt-0" }, {
                        default: withCtx(() => [
                          createVNode(VSwitch, {
                            "model-value": site.enabled,
                            label: site.enabled ? "On" : "Off",
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(site, !!v)
                          }, null, 8, ["model-value", "label", "onUpdate:modelValue"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-open-in-new",
                            color: "info",
                            onClick: ($event) => openSiteUrl(site)
                          }, {
                            default: withCtx(() => [..._cache[33] || (_cache[33] = [
                              createTextVNode(" Browse ", -1)
                            ])]),
                            _: 1
                          }, 8, ["onClick"]),
                          createVNode(VBtn, {
                            variant: "text",
                            size: "small",
                            icon: "mdi-pencil",
                            color: "primary",
                            onClick: ($event) => openEditDialog(site)
                          }, null, 8, ["onClick"]),
                          createVNode(VBtn, {
                            variant: "text",
                            size: "small",
                            icon: "mdi-delete",
                            color: "error",
                            onClick: ($event) => confirmRemove(site)
                          }, null, 8, ["onClick"])
                        ]),
                        _: 2
                      }, 1024)
                    ]),
                    _: 2
                  }, 1032, ["class"])
                ]),
                _: 2
              }, 1024);
            }), 128))
          ]),
          _: 1
        }),
        unref(siteStore).loading ? (openBlock(), createBlock(VRow, { key: 1 }, {
          default: withCtx(() => [
            (openBlock(), createElementBlock(Fragment, null, renderList(3, (n) => {
              return createVNode(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: n
              }, {
                default: withCtx(() => [
                  createVNode(VSkeletonLoader, {
                    type: "card",
                    class: "skeleton-card"
                  })
                ]),
                _: 1
              });
            }), 64))
          ]),
          _: 1
        })) : createCommentVNode("", true),
        !unref(siteStore).loading && unref(siteStore).sites.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_23, [
          createBaseVNode("div", _hoisted_24, [
            createVNode(VIcon, {
              size: "72",
              color: "medium-emphasis"
            }, {
              default: withCtx(() => [..._cache[34] || (_cache[34] = [
                createTextVNode("mdi-web-off", -1)
              ])]),
              _: 1
            })
          ]),
          _cache[36] || (_cache[36] = createBaseVNode("h3", { class: "text-h5 font-weight-medium mb-2" }, "No sites configured", -1)),
          _cache[37] || (_cache[37] = createBaseVNode("p", { class: "text-body-1 text-medium-emphasis mb-6" }, "Deploy your first static website to get started", -1)),
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            "prepend-icon": "mdi-plus",
            variant: "elevated",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[35] || (_cache[35] = [
              createTextVNode(" Create Your First Site ", -1)
            ])]),
            _: 1
          })
        ])) : createCommentVNode("", true),
        createVNode(VDialog, {
          modelValue: showDialog.value,
          "onUpdate:modelValue": _cache[14] || (_cache[14] = ($event) => showDialog.value = $event),
          "max-width": "800",
          transition: "dialog-bottom-transition",
          scrim: "rgba(0,0,0,0.5)"
        }, {
          default: withCtx(() => [
            createVNode(VCard, { class: "dialog-card" }, {
              default: withCtx(() => [
                createBaseVNode("div", {
                  class: normalizeClass(["dialog-top-bar", editingSite.value ? "bg-primary" : "bg-success"])
                }, null, 2),
                createVNode(VCardTitle, { class: "d-flex align-center pt-4 px-6" }, {
                  default: withCtx(() => [
                    createVNode(VAvatar, {
                      color: editingSite.value ? "primary" : "success",
                      size: "36",
                      variant: "tonal",
                      class: "mr-3"
                    }, {
                      default: withCtx(() => [
                        createVNode(VIcon, {
                          size: "20",
                          color: "white"
                        }, {
                          default: withCtx(() => [
                            createTextVNode(toDisplayString(editingSite.value ? "mdi-pencil" : "mdi-plus"), 1)
                          ]),
                          _: 1
                        })
                      ]),
                      _: 1
                    }, 8, ["color"]),
                    createBaseVNode("div", null, [
                      createBaseVNode("div", _hoisted_25, toDisplayString(editingSite.value ? "Edit Site" : "Create New Site"), 1),
                      createBaseVNode("div", _hoisted_26, toDisplayString(editingSite.value ? "Modify site configuration" : "Configure and deploy a new static website"), 1)
                    ])
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createBaseVNode("div", _hoisted_27, [
                      createBaseVNode("div", _hoisted_28, [
                        createVNode(VIcon, {
                          size: "18",
                          color: "primary",
                          class: "mr-2"
                        }, {
                          default: withCtx(() => [..._cache[38] || (_cache[38] = [
                            createTextVNode("mdi-information", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[39] || (_cache[39] = createBaseVNode("span", { class: "text-body-2 font-weight-bold" }, "Basic Information", -1))
                      ]),
                      createVNode(VRow, { dense: "" }, {
                        default: withCtx(() => [
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.name,
                                "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => form.value.name = $event),
                                label: "Name",
                                variant: "outlined",
                                placeholder: "my-site",
                                hint: "Unique identifier for the site",
                                "persistent-hint": "",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.title,
                                "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => form.value.title = $event),
                                label: "Title",
                                variant: "outlined",
                                placeholder: "My Site",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, { cols: "12" }, {
                            default: withCtx(() => [
                              createVNode(VTextarea, {
                                modelValue: form.value.description,
                                "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.description = $event),
                                label: "Description",
                                variant: "outlined",
                                rows: "2",
                                placeholder: "Site description...",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          })
                        ]),
                        _: 1
                      })
                    ]),
                    createBaseVNode("div", _hoisted_29, [
                      createBaseVNode("div", _hoisted_30, [
                        createVNode(VIcon, {
                          size: "18",
                          color: "primary",
                          class: "mr-2"
                        }, {
                          default: withCtx(() => [..._cache[40] || (_cache[40] = [
                            createTextVNode("mdi-folder-cog", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[41] || (_cache[41] = createBaseVNode("span", { class: "text-body-2 font-weight-bold" }, "Deployment", -1))
                      ]),
                      createVNode(VRow, { dense: "" }, {
                        default: withCtx(() => [
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VMenu, {
                                modelValue: pathMenuOpen.value,
                                "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => pathMenuOpen.value = $event),
                                "close-on-content-click": false,
                                location: "bottom"
                              }, {
                                activator: withCtx(({ props }) => [
                                  createVNode(VTextField, mergeProps({
                                    modelValue: form.value.path,
                                    "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => form.value.path = $event),
                                    label: "Path",
                                    variant: "outlined",
                                    placeholder: "Select folder",
                                    "append-icon": "mdi-folder",
                                    readonly: "",
                                    density: "comfortable"
                                  }, props), null, 16, ["modelValue"])
                                ]),
                                default: withCtx(() => [
                                  createVNode(VCard, {
                                    "max-height": "320",
                                    class: "overflow-y-auto folder-picker",
                                    "min-width": "320"
                                  }, {
                                    default: withCtx(() => [
                                      createBaseVNode("div", _hoisted_31, [
                                        createVNode(VIcon, {
                                          size: "18",
                                          color: "primary",
                                          class: "mr-2"
                                        }, {
                                          default: withCtx(() => [..._cache[42] || (_cache[42] = [
                                            createTextVNode("mdi-folder-open", -1)
                                          ])]),
                                          _: 1
                                        }),
                                        _cache[44] || (_cache[44] = createBaseVNode("span", { class: "text-caption font-weight-bold" }, "Select Folder", -1)),
                                        createVNode(VSpacer),
                                        form.value.path ? (openBlock(), createBlock(VBtn, {
                                          key: 0,
                                          variant: "text",
                                          size: "x-small",
                                          color: "error",
                                          onClick: _cache[4] || (_cache[4] = ($event) => form.value.path = "")
                                        }, {
                                          default: withCtx(() => [..._cache[43] || (_cache[43] = [
                                            createTextVNode(" Clear ", -1)
                                          ])]),
                                          _: 1
                                        })) : createCommentVNode("", true)
                                      ]),
                                      createVNode(VDivider),
                                      loadingFolders.value ? (openBlock(), createBlock(VProgressLinear, {
                                        key: 0,
                                        indeterminate: "",
                                        color: "primary"
                                      })) : folderTree.value.length === 0 ? (openBlock(), createBlock(VList, {
                                        key: 1,
                                        density: "compact"
                                      }, {
                                        default: withCtx(() => [
                                          createVNode(VListItem, {
                                            title: "No folders available",
                                            density: "compact"
                                          })
                                        ]),
                                        _: 1
                                      })) : (openBlock(), createBlock(VList, {
                                        key: 2,
                                        density: "compact"
                                      }, {
                                        default: withCtx(() => [
                                          createVNode(FolderTreeItem, {
                                            nodes: folderTree.value,
                                            "current-path": form.value.path,
                                            "expanded-paths": expandedPaths.value,
                                            onSelect: _cache[5] || (_cache[5] = (p) => {
                                              selectPath(p);
                                              pathMenuOpen.value = false;
                                            }),
                                            onToggle: toggleExpand
                                          }, null, 8, ["nodes", "current-path", "expanded-paths"])
                                        ]),
                                        _: 1
                                      }))
                                    ]),
                                    _: 1
                                  })
                                ]),
                                _: 1
                              }, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.entry_point,
                                "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => form.value.entry_point = $event),
                                label: "Entry Point",
                                variant: "outlined",
                                placeholder: "index.html",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VSelect, {
                                modelValue: form.value.component,
                                "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => form.value.component = $event),
                                label: "Component",
                                variant: "outlined",
                                items: componentOptions,
                                density: "comfortable",
                                hint: "WASM component for serving static files",
                                "persistent-hint": ""
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          })
                        ]),
                        _: 1
                      })
                    ]),
                    createBaseVNode("div", _hoisted_32, [
                      createBaseVNode("div", _hoisted_33, [
                        createVNode(VIcon, {
                          size: "18",
                          color: "primary",
                          class: "mr-2"
                        }, {
                          default: withCtx(() => [..._cache[45] || (_cache[45] = [
                            createTextVNode("mdi-tune", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[46] || (_cache[46] = createBaseVNode("span", { class: "text-body-2 font-weight-bold" }, "Configuration", -1))
                      ]),
                      createVNode(VRow, { dense: "" }, {
                        default: withCtx(() => [
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VSwitch, {
                                modelValue: form.value.enable_auth,
                                "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => form.value.enable_auth = $event),
                                label: "Enable Authentication",
                                color: "warning",
                                "hide-details": "",
                                density: "compact"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VSwitch, {
                                modelValue: form.value.enabled,
                                "onUpdate:modelValue": _cache[10] || (_cache[10] = ($event) => form.value.enabled = $event),
                                label: "Enabled",
                                color: "success",
                                "hide-details": "",
                                density: "compact"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          form.value.enable_auth ? (openBlock(), createBlock(VCol, {
                            key: 0,
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.auth_user,
                                "onUpdate:modelValue": _cache[11] || (_cache[11] = ($event) => form.value.auth_user = $event),
                                label: "Auth User",
                                variant: "outlined",
                                placeholder: "admin",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          })) : createCommentVNode("", true),
                          form.value.enable_auth ? (openBlock(), createBlock(VCol, {
                            key: 1,
                            cols: "12",
                            md: "6"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.auth_pass,
                                "onUpdate:modelValue": _cache[12] || (_cache[12] = ($event) => form.value.auth_pass = $event),
                                label: "Auth Password",
                                variant: "outlined",
                                type: "password",
                                placeholder: "Enter password",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          })) : createCommentVNode("", true)
                        ]),
                        _: 1
                      })
                    ])
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, { class: "pa-4 pt-0" }, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "outlined",
                      onClick: _cache[13] || (_cache[13] = ($event) => showDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[47] || (_cache[47] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: editingSite.value ? "primary" : "success",
                      variant: "elevated",
                      onClick: handleSave,
                      loading: saving.value,
                      disabled: !form.value.name.trim(),
                      class: "ml-2"
                    }, {
                      default: withCtx(() => [
                        createVNode(VIcon, {
                          start: "",
                          size: "18"
                        }, {
                          default: withCtx(() => [
                            createTextVNode(toDisplayString(editingSite.value ? "mdi-content-save" : "mdi-plus-circle"), 1)
                          ]),
                          _: 1
                        }),
                        createTextVNode(" " + toDisplayString(editingSite.value ? "Save Changes" : "Create Site"), 1)
                      ]),
                      _: 1
                    }, 8, ["color", "loading", "disabled"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: deleteDialog.value,
          "onUpdate:modelValue": _cache[16] || (_cache[16] = ($event) => deleteDialog.value = $event),
          "max-width": "420",
          transition: "dialog-bottom-transition",
          scrim: "rgba(0,0,0,0.5)"
        }, {
          default: withCtx(() => [
            createVNode(VCard, { class: "dialog-card" }, {
              default: withCtx(() => [
                _cache[57] || (_cache[57] = createBaseVNode("div", { class: "dialog-top-bar bg-error" }, null, -1)),
                createVNode(VCardTitle, { class: "d-flex align-center pt-4" }, {
                  default: withCtx(() => [
                    createVNode(VIcon, {
                      color: "error",
                      size: "24",
                      class: "mr-2"
                    }, {
                      default: withCtx(() => [..._cache[48] || (_cache[48] = [
                        createTextVNode("mdi-delete-alert", -1)
                      ])]),
                      _: 1
                    }),
                    _cache[49] || (_cache[49] = createTextVNode(" Delete Site ", -1))
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a, _b;
                    return [
                      createBaseVNode("div", _hoisted_34, [
                        createBaseVNode("p", _hoisted_35, [
                          _cache[50] || (_cache[50] = createTextVNode(" Are you sure you want to delete ", -1)),
                          createBaseVNode("strong", _hoisted_36, toDisplayString(((_a = deleteTarget.value) == null ? void 0 : _a.title) || ((_b = deleteTarget.value) == null ? void 0 : _b.name)), 1),
                          _cache[51] || (_cache[51] = createTextVNode("? ", -1))
                        ]),
                        createBaseVNode("p", _hoisted_37, [
                          createVNode(VIcon, {
                            size: "14",
                            color: "error",
                            class: "mr-1"
                          }, {
                            default: withCtx(() => [..._cache[52] || (_cache[52] = [
                              createTextVNode("mdi-alert-circle", -1)
                            ])]),
                            _: 1
                          }),
                          _cache[53] || (_cache[53] = createTextVNode(" This action cannot be undone. All site configuration will be permanently removed. ", -1))
                        ])
                      ])
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, { class: "pa-4 pt-0" }, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "outlined",
                      onClick: _cache[15] || (_cache[15] = ($event) => deleteDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[54] || (_cache[54] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      variant: "elevated",
                      onClick: handleDelete,
                      loading: deleting.value,
                      class: "ml-2"
                    }, {
                      default: withCtx(() => [
                        createVNode(VIcon, {
                          start: "",
                          size: "18"
                        }, {
                          default: withCtx(() => [..._cache[55] || (_cache[55] = [
                            createTextVNode("mdi-delete", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[56] || (_cache[56] = createTextVNode(" Delete Site ", -1))
                      ]),
                      _: 1
                    }, 8, ["loading"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"])
      ]);
    };
  }
});
const SitesPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-abccfcec"]]);
export {
  SitesPage as default
};
