/**
 * Load product catalog and details.
 */

import { readFileSync, existsSync } from 'node:fs'
import { join } from 'node:path'
import { resolveCompanyPath, resolveRegistryPath, pathExists } from './paths.js'
import type { ProductDetails, ProductDocumentation, ProductCatalog } from './types.js'

/**
 * Load the product catalog (products/index.json) for a company.
 */
export const loadProductCatalog = (companyId: string): ProductCatalog | null => {
  const catalogPath = resolveCompanyPath(companyId, 'products', 'index.json')

  if (!pathExists(catalogPath)) {
    console.warn(`Product catalog not found: ${catalogPath}`)
    return null
  }

  try {
    const content = readFileSync(catalogPath, 'utf-8').replace(/^\uFEFF/, '')
    return JSON.parse(content) as ProductCatalog
  } catch (error) {
    console.error(`Failed to load product catalog for ${companyId}:`, error)
    return null
  }
}

/**
 * Load product details from a path.
 * Path should be relative to company root (e.g., "products/podcast/details.json")
 */
export const loadProduct = (companyId: string, detailsPath: string): ProductDetails | null => {
  const fullPath = resolveCompanyPath(companyId, detailsPath)

  if (!pathExists(fullPath)) {
    console.warn(`Product details not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')
    return JSON.parse(content) as ProductDetails
  } catch (error) {
    console.error(`Failed to load product details: ${detailsPath}`, error)
    return null
  }
}

/**
 * Load product documentation (markdown) from a path.
 * Path should be relative to company root (e.g., "products/podcast/documentation.md")
 */
export const loadProductDocs = (
  companyId: string,
  docsPath: string
): ProductDocumentation | null => {
  const fullPath = resolveCompanyPath(companyId, docsPath)

  if (!pathExists(fullPath)) {
    console.warn(`Product documentation not found: ${fullPath}`)
    return null
  }

  try {
    const content = readFileSync(fullPath, 'utf-8').replace(/^\uFEFF/, '')

    // Parse front matter if present
    const frontMatterMatch = content.match(/^---\n([\s\S]*?)\n---\n?/)
    const metadata: ProductDocumentation['metadata'] = {}

    if (frontMatterMatch) {
      const frontMatter = frontMatterMatch[1]
      const productIdMatch = frontMatter.match(/^productId:\s*(.+)$/m)
      const companyIdMatch = frontMatter.match(/^companyId:\s*(.+)$/m)
      const typeMatch = frontMatter.match(/^type:\s*(.+)$/m)
      const generatedMatch = frontMatter.match(/^generatedAt:\s*(.+)$/m)

      if (productIdMatch) metadata.productId = productIdMatch[1].trim()
      if (companyIdMatch) metadata.companyId = companyIdMatch[1].trim()
      if (typeMatch) metadata.type = typeMatch[1].trim()
      if (generatedMatch) metadata.generatedAt = generatedMatch[1].trim()
    }

    return { content, metadata }
  } catch (error) {
    console.error(`Failed to load product documentation: ${docsPath}`, error)
    return null
  }
}

/**
 * Load product by ID using the product catalog.
 */
export const loadProductById = (companyId: string, productId: string): ProductDetails | null => {
  const catalog = loadProductCatalog(companyId)
  if (!catalog) {
    return null
  }

  const product = catalog.products[productId]
  if (!product) {
    console.warn(`Product not found in catalog: ${productId}`)
    return null
  }

  return loadProduct(companyId, product.details)
}

/**
 * Load product docs by ID using the product catalog.
 */
export const loadProductDocsById = (
  companyId: string,
  productId: string
): ProductDocumentation | null => {
  const catalog = loadProductCatalog(companyId)
  if (!catalog) {
    return null
  }

  const product = catalog.products[productId]
  if (!product || !product.documentation) {
    console.warn(`Product documentation not found in catalog: ${productId}`)
    return null
  }

  return loadProductDocs(companyId, product.documentation)
}

/**
 * Load a product with its documentation.
 */
export const loadProductWithDocs = (
  companyId: string,
  detailsPath: string,
  docsPath?: string
): { details: ProductDetails | null; documentation: ProductDocumentation | null } => {
  return {
    details: loadProduct(companyId, detailsPath),
    documentation: docsPath ? loadProductDocs(companyId, docsPath) : null
  }
}

/**
 * Load a product by ID with its documentation.
 */
export const loadProductWithDocsById = (
  companyId: string,
  productId: string
): { details: ProductDetails | null; documentation: ProductDocumentation | null } => {
  const catalog = loadProductCatalog(companyId)
  if (!catalog) {
    return { details: null, documentation: null }
  }

  const product = catalog.products[productId]
  if (!product) {
    return { details: null, documentation: null }
  }

  return loadProductWithDocs(companyId, product.details, product.documentation)
}
